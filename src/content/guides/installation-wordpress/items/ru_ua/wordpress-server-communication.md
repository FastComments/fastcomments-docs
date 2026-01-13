Для работы плагина token сохраняется в вашей базе данных WordPress и также в вашей учётной записи FastComments. Когда плагин отправляет запрос на наши серверы, он предоставляет
этот token.

Вы можете просмотреть все интеграции, авторизованные для вашей учётной записи FastComments, [здесь](https://fastcomments.com/auth/my-account/manage-data/integrations).

Вся коммуникация осуществляется по HTTPS.

Вся коммуникация *исходит* с вашего сервера WordPress *на* FastComments.com, включая синхронизацию *обратно* в вашу установку WordPress, поскольку она реализована
путём [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) через настроенный в вашей установке WordPress [cron](https://developer.wordpress.org/plugins/cron/).