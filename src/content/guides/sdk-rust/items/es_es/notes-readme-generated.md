### Identificadores de transmisión

Verás que debes pasar un `broadcastId` en algunas llamadas a la API. Cuando recibas eventos, obtendrás este ID de vuelta, de modo que sepas ignorar el evento si planeas aplicar los cambios de forma optimista en el cliente
(lo que probablemente querrás hacer, ya que ofrece la mejor experiencia). Pasa un UUID aquí. El ID debe ser lo suficientemente único para no ocurrir dos veces en una sesión del navegador.