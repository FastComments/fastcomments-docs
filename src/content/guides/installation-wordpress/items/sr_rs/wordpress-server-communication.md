Да би додатак функционисао, token се чува у вашој WordPress бази података и такође на вашем FastComments налогу. Када додатак упути захтев нашим серверима, он прослеђује
тај token.

Све интеграције овлашћене за ваш FastComments налог можете погледати [овде](https://fastcomments.com/auth/my-account/manage-data/integrations).

Сва комуникација се обавља преко HTTPS-а.

Сва комуникација је *излазна* са вашег WordPress сервера *према* FastComments.com, укључујући и синхронизацију *назад* на вашу WordPress инсталацију како је реализована
путем [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) из [cron](https://developer.wordpress.org/plugins/cron/) подешавања у вашој WordPress инсталацији.