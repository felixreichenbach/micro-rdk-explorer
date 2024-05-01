use micro_rdk::common::board::{Board,BoardType};
use micro_rdk::common::config::ConfigType;
use micro_rdk::common::generic::{DoCommand, GenericError};
use micro_rdk::common::registry::{
    get_board_from_dependencies, ComponentRegistry, Dependency, RegistryError,
};
use micro_rdk::common::sensor::{GenericReadingsResult, Readings, Sensor, SensorError, SensorType};
use micro_rdk::common::status::{Status, StatusError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use micro_rdk::google::protobuf::{value::Kind, Struct, Value};

pub fn register_models(registry: &mut ComponentRegistry) -> Result<(), RegistryError> {
    registry.register_sensor("my_sensor", &MySensor::from_config)
}

pub struct MySensor {
    board: BoardType,
}

impl Sensor for MySensor {}

impl MySensor {
    pub fn from_config(_cfg: ConfigType, deps: Vec<Dependency>) -> Result<SensorType, SensorError> {
        let board_handle =
            get_board_from_dependencies(deps).expect("failed to get board from dependencies");
        Ok(Arc::new(Mutex::new(MySensor {
            board: board_handle,
        })))
    }
}

impl Readings for MySensor {
    fn get_generic_readings(&mut self) -> Result<GenericReadingsResult, SensorError> {
        let mut x = HashMap::new();
        let value = Value {
            kind: Option::Some(Kind::NumberValue(911 as f64)),
        };
        x.insert(String::from("sensor_value"), value);
        Ok(x)
    }
}

impl Status for MySensor {
    fn get_status(&self) -> Result<Option<micro_rdk::google::protobuf::Struct>, StatusError> {
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}

impl DoCommand for MySensor {
    fn do_command(
        &mut self,
        command_struct: Option<Struct>,
    ) -> Result<Option<micro_rdk::google::protobuf::Struct>, GenericError> {
        let mut res = HashMap::new();
        if let Some(command_struct) = command_struct.as_ref() {
            for (key, val) in &command_struct.fields {
                match key.as_str() {
                    "ping" => {
                        res.insert(
                            "ping".to_string(),
                            Value {
                                kind: Some(Kind::StringValue("pinged".to_string())),
                            },
                        );
                    }
                    "echo" => {
                        res.insert("echoed".to_string(), val.to_owned());
                    }
                    "board" => {
                        let mut pin: i32 = 0;
                        for value in &val.kind {
                            pin = match value {
                                Kind::NullValue(_) => todo!(),
                                Kind::NumberValue(pin) => *pin as i32,
                                Kind::StringValue(_) => todo!(),
                                Kind::BoolValue(_) => todo!(),
                                Kind::StructValue(_) => todo!(),
                                Kind::ListValue(_) => todo!(),
                            };
                        }
                        let value = match self.board.get_gpio_level(pin) {
                            Ok(value) => Some(Kind::BoolValue(value)),
                            Err(_e) => return Err(GenericError::MethodUnimplemented("do_command")),
                        };
                        res.insert("pin".to_string(), Value { kind: value });
                    }
                    _ => {}
                };
            }
        }
        Ok(Some(Struct { fields: res }))
    }
}
