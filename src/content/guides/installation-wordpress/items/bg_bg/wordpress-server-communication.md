За да работи плъгинът, в базата данни на вашия WordPress и във вашия акаунт в FastComments се записва токен. Когато плъгинът прави заявка към нашите сървъри, той предоставя
този токен.

Можете да видите всички интеграции, оторизирани за вашия акаунт в FastComments [тук](https://fastcomments.com/auth/my-account/manage-data/integrations).

Всяка комуникация се извършва чрез HTTPS.

Всяката комуникация е *изходяща* от вашия WordPress сървър *към* FastComments.com, включително синхронизацията *обратно* към вашата WordPress инсталация, тъй като тя е реализирана
чрез [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) от [cron](https://developer.wordpress.org/plugins/cron/) настройка във вашата WordPress инсталация.