### IDs de Broadcast

Verás que se supone que debes pasar un `broadcastId` en algunas llamadas a la API. Cuando recibas eventos, obtendrás este ID de vuelta, así sabrás ignorar el evento si planeas aplicar cambios de forma optimista en el cliente
(lo cual probablemente querrás hacer ya que ofrece la mejor experiencia). Pasa un UUID aquí. El ID debe ser lo suficientemente único como para no ocurrir dos veces en una sesión del navegador.