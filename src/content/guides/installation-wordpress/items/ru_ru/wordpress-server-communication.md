---
Для работы плагина токен сохраняется в базе данных вашего WordPress и также в вашей учетной записи FastComments. Когда плагин делает запросы к нашим серверам, он предоставляет этот токен.

Вы можете просмотреть все интеграции, авторизованные для вашей учетной записи FastComments, [здесь](https://fastcomments.com/auth/my-account/manage-data/integrations).

Вся связь осуществляется через HTTPS.

Вся коммуникация является *исходящей* с вашего сервера WordPress *к* FastComments.com, включая синхронизацию *назад* в вашу установку WordPress, так как она реализована через [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) с помощью [cron](https://developer.wordpress.org/plugins/cron/) в вашей установке WordPress.
---