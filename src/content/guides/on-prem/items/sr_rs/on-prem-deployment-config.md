FastComments користи променљиве окружења за конфигурацију. Следећа листа наводи све подржане променљиве које су релевантне за On-Prem.


| Променљива                     | Подразумевано               | Информације                                                                                                                                        | Обавезно | Примери или валидне вредности                         |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Тип окружења.                                                                                                                                      | Yes      | production, dev                                       |
| MONGO_URI                      |                             | DB Connection URI.                                                                                                                                 | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Омогућава коришћење SSL-а за повезивање са базом података.                                                                                         | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Омогућава валидацију сертификата у односу на CA када се повезује на Mongo.                                                                           | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem фајл.                                                                                                                             | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email на који би требало да стижу важне системске обавештења.                                                                                      | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Салт за хеширање IP адреса.                                                                                                                       | Yes      |                                                       |
| SESSION_SECRET                 |                             | Кључ који се користи за потписивање сесија.                                                                                                        | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | Кључ који се користи за потписивање/хеширање сесија у складишту. Мора се разликовати од SESSION_SECRET.                                              | Yes      |                                                       |
| HOSTNAME                       |                             | Име хоста на којем је FastComments размењен (админски панел итд). НЕ сме да садржи порт или протокол.                                               | Yes      | example.com                                           |
| HOST_ADDR                      |                             | Приступачан URI где је FastComments размењен (админски панел итд).                                                                                 | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Путања на локалном фајл систему где се налази email конфигурација (SMTP, мапирање домена/провајдера итд).                                           | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Заглавље "From Name" у имејлу.                                                                                                                     | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Лого у подножју имејла.                                                                                                                            | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Замена за "defaultTransport" у EMAIL_CONFIG_PATH. Корисно за распоређивање исте конфигурационе датотеке у различитим окружењима.                      | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ИД вашег налога на fastcomments.com. Користи се за регистрацију вашег лиценцног кључа.                                                              | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Лиценцни кључ за on-prem инсталацију.                                                                                                             | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API кључ. Ако није наведен, требало би да направите правило конфигурације које онемогућава бирач GIF-ова.                                      | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Користи се за Giphy интеграцију. Такође може бити промењен правилима прилагођавања виџета.                                                           | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Користи се за OpenAI могућности као што су опционална GPT-базирана детекција спама.                                                                 | No       |                                                       |
| CDN_HOST_ADDR                  |                             | Име хоста са којег ће се преузимати ресурси. По подразумеваној вредности користи се вредност HOSTNAME.                                               | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Име хоста са којег се преузимају велики фајлови (као што су експорти). По подразумеваној вредности користи се вредност CDN_HOST_ADDR.                  | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Где треба да се чувају велики фајлови, као што су експорти.                                                                                         | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Име хоста са којег треба да се шаљу имејлови.                                                                                                       | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Име fastcomments колачића.                                                                                                                         | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Вредност поља "hostname" у колачићу. Препоручује се да има префикс са тачком.                                                                       | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Користи се за корисничка отпремања фајлова, аватаре итд. По подразумеваној вредности користи локални фајл систем ако није дефинисано.                  | No       |                                                       |
| S3_SECRET_KEY                  |                             | Користи се за корисничка отпремања фајлова, аватаре итд.                                                                                            | No       |                                                       |
| S3_REGION                      |                             | Користи се за корисничка отпремања фајлова, аватаре итд.                                                                                            | No       |                                                       |
| S3_BUCKET                      |                             | Користи се за корисничка отпремања фајлова, аватаре итд.                                                                                            | No       |                                                       |
| S3_HOST                        |                             | Користи се за корисничка отпремања фајлова, аватаре итд.                                                                                            | No       |                                                       |
| CACHE_DIR                      |                             | Локација за чување опционалног офлајн кеша, за случај када БД није доступна. Периодично се освежава са 100 најактивнијих тема коментара.               | No       |                                                       |
| BACKUP_DIR                     |                             | Локација за чување података за случај када БД није доступна. Ако се коментар пошаље када је БД недоступна, он се овде чува и касније обрађује.         | No       |                                                       |

Имајте у виду да све променљиве везане за домене користе суфиксе `_HOST` или `_ADDR`. Разлика је:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Путanja у `EMAIL_CONFIG_PATH` треба да садржи путању до JSON фајла са следећим примером формата:

[inline-code-attrs-start title = 'Конфигурација е-поште'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "defaultDKIM": {
        "domainName": "mycompany.org",
        "keySelector": "2024",
        "privateKey": "-----BEGIN PRIVATE KEY-----\nABCDEFG\n-----END PRIVATE KEY-----"
    },
    "providerTransports": {
        "yahoo.com": "specialTransport"
    },
    "defaultTransport": "mailgun",
    "transports": {
        "mailgun": {
            "host": "smtp.mailgun.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@somewhere.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        },
        "specialTransport": {
            "host": "smtp.someplace.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@example.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        }
    }
}
[inline-code-end]

У горњем примеру дефинишемо подразумевани `SMTP` имејл транспорт под називом `mailgun`. Такође дефинишемо посебан транспорт који користимо специфично за имејлове са `@yahoo.com`. У неким сценаријима је пожељно користити специфичног провајдера или IP за слање за одређени домен како би се подесила испорука. Ово је опционално.

### DocumentDB

Када се повезујете на `DocumentDB`, желећете да наведете `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` да бисте били компатибилни са подразумеваним подешавањима.