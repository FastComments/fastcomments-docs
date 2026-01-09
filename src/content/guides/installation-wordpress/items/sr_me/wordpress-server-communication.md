Да би додатак функционисао, token се чува у вашој WordPress бази података и такође на вашем FastComments налогу. Када додатак пошаље захтјев нашим серверима, он доставља
овај token.

Све интеграције овлашћене за ваш FastComments налог можете погледати [овде](https://fastcomments.com/auth/my-account/manage-data/integrations).

Сва комуникација се обавља путем HTTPS-а.

Сва комуникација је *outbound* са вашег WordPress сервера *to* FastComments.com, укључујући синхронизацију *back* ка вашој WordPress инсталацији, јер је она имплементирана путем [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) из [cron](https://developer.wordpress.org/plugins/cron/) подешавања у вашој WordPress инсталацији.