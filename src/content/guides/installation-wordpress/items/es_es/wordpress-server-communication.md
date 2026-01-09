---
Para que el complemento funcione, se guarda un token en la base de datos de tu WordPress y también en tu cuenta de FastComments. Cuando el complemento realiza solicitudes a nuestros servidores, proporciona
este token.

Puedes ver todas las integraciones autorizadas en tu cuenta de FastComments [aquí](https://fastcomments.com/auth/my-account/manage-data/integrations).

Toda la comunicación se realiza a través de HTTPS.

Toda la comunicación es *saliente* desde tu servidor WordPress *hacia* FastComments.com, incluyendo la sincronización *de vuelta* a tu instalación de WordPress ya que se implementa
mediante [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) desde una [cron](https://developer.wordpress.org/plugins/cron/) configurada en tu instalación de WordPress.

---