Cuando un usuario visita una entidad con el campo FastComments habilitado:

1. El widget JavaScript de FastComments se carga desde la CDN.
2. Si SSO está configurado, la identidad de Drupal del usuario se transmite a FastComments.
3. Una alternativa `<noscript>` proporciona comentarios renderizados en el servidor para usuarios sin JavaScript (solo en los modos Live Comments y Streaming Chat).