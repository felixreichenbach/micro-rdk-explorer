# Viam Micro-RDK ESP32 Starter Kit

## Machine Configuration

```json
{
  "components": [
    {
      "name": "mysensor",
      "namespace": "rdk",
      "type": "sensor",
      "model": "my_sensor",
      "attributes": {}
    },
    {
      "name": "board",
      "namespace": "rdk",
      "type": "board",
      "model": "esp32",
      "attributes": {
        "pins": [
          15,
          33,
          32
        ],
        "analogs": [
          {
            "pin": "33",
            "name": "sensor"
          }
        ]
      }
    }
  ]
}```
