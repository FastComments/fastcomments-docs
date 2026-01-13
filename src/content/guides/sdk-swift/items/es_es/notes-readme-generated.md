### IDs de transmisión

Verás que debes pasar un `broadcastId` en algunas llamadas a la API. Cuando recibas eventos, obtendrás este ID de vuelta, de modo que sabrás ignorar el evento si planeas aplicar cambios de forma optimista en el cliente (lo cual probablemente querrás hacer, ya que ofrece la mejor experiencia). Pasa un UUID aquí. El ID debe ser lo suficientemente único para no repetirse en una sesión.

```swift
let broadcastId = UUID().uuidString
```