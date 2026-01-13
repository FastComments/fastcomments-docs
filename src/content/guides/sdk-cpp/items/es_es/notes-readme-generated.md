### IDs de difusión

Verás que se supone que debes pasar un `broadcastId` en algunas llamadas de la API. Cuando recibas eventos, obtendrás este ID de vuelta, por lo que sabrás ignorar el evento si planeas aplicar cambios de forma optimista en el cliente (lo que probablemente querrás hacer, ya que ofrece la mejor experiencia). Pasa un UUID aquí. El ID debe ser lo suficientemente único para no ocurrir dos veces en una sesión del navegador.

### SSO (Inicio de sesión único)

Para ejemplos de SSO, consulte más abajo.