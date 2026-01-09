FastComments користи варијабле окружења за конфигурацију. Сљедећа листа наводи све подржане варијабле које су релевантне за On-Prem.

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Тип окружења.                                                                                                                                      | Да       | production, dev                                       |
| MONGO_URI                      |                             | DB URI за повезивање.                                                                                                                              | Да       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Омогућава коришћење SSL-а за повезивање са базом података.                                                                                         | Не       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Омогућава валидацију сертификата према CA при повезивању на Mongo.                                                                                  | Не       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem фајл.                                                                                                                             | Не       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Е-пошта на коју би требало стизати важне обавијести везане за систем.                                                                                | Не       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Сол за хеширање IP адреса.                                                                                                                         | Да       |                                                       |
| SESSION_SECRET                 |                             | Кључ који се користи за потписивање сесија.                                                                                                        | Да       |                                                       |
| SESSION_STORE_SECRET           |                             | Кључ који се користи за потписивање/хаширање сесија у складишту. Мора бити различит од SESSION_SECRET.                                               | Да       |                                                       |
| HOSTNAME                       |                             | Хостнаме гдје је FastComments развијен (админ контролна плоча итд). НЕ ТРЕБА садржавати порт или протокол.                                           | Да       | example.com                                           |
| HOST_ADDR                      |                             | Доступан URI гдје је FastComments развијен (админ контролна плоча итд).                                                                             | Да       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Путања на локалном фајл систему гдје се налази конфигурација е-поште (SMTP, мапирања домена/провајдера, итд).                                         | Да       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Заглавље „From Name“ у е-порукама.                                                                                                                  | Не       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Лого у футеру е-поруке.                                                                                                                             | Не       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Замјена за "defaultTransport" у EMAIL_CONFIG_PATH. Корисно за коришћење истог конфигурационог фајла у различитим енвима.                             | Не       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ИД вашег налога на fastcomments.com. Користи се за регистрацију вашег лиценцног кључа.                                                               | Не       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-prem лиценцни кључ.                                                                                                                             | Не       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API кључ. Ако није наведен, требало би креирати правило у конфигурацији које онемогућава бирач гифова.                                          | Не       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Користи се за интеграцију са giphy-јем. Такођер се може преоптеретити правилима за прилагођавање виџета.                                             | Не       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Користи се за OpenAI функције попут опционог GPT-базираног детекције спама.                                                                          | Не       |                                                       |
| CDN_HOST_ADDR                  |                             | Хостнаме са ког ће се преузимати ресурси. Подразумевано је вриједност HOSTNAME.                                                                    | Не       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Хостнаме са ког ће се преузимати велики фајлови (нпр. извези). Подразумевано је вриједност CDN_HOST_ADDR.                                            | Не       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Гдје треба чувати велике фајлове, као што су извези.                                                                                               | Не       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Хостнаме са ког би требало слати е-поруке.                                                                                                          | Не       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Име fastcomments колачића.                                                                                                                         | Не       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Вриједност поља "hostname" у колачићу. Препоручује се да има префикс тачке.                                                                          | Не       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Користи се за корисничке отпреме фајлова, аватаре итд. Подразумјевано је локални фајл систем ако није дефинисано.                                      | Не       |                                                       |
| S3_SECRET_KEY                  |                             | Користи се за корисничке отпреме фајлова, аватаре итд.                                                                                              | Не       |                                                       |
| S3_REGION                      |                             | Користи се за корисничке отпреме фајлова, аватаре итд.                                                                                              | Не       |                                                       |
| S3_BUCKET                      |                             | Користи се за корисничке отпреме фајлова, аватаре итд.                                                                                              | Не       |                                                       |
| S3_HOST                        |                             | Користи се за корисничке отпреме фајлова, аватаре итд.                                                                                              | Не       |                                                       |
| CACHE_DIR                      |                             | Локација за чување опционалне офлајн кеш меморије, за случај кад DB није доступна. Периодично се ажурира са топ 100 тема коментара.                      | Не       |                                                       |
| BACKUP_DIR                     |                             | Локација за чување података за случај кад DB није доступна. Ако се коментар пошаље док DB није доступан, иде овдје и обрађује се касније.               | Не       |                                                       |

Имајте у виду да све варијабле везане за домене користе постфикс `_HOST` или `_ADDR`. Разлика је:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Путања у `EMAIL_CONFIG_PATH` треба да садржи путању до JSON фајла са сљедећим примјер форматом:

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

У горњем примјеру дефинишемо подразумевани `SMTP` транспорт е-поште који се зове `mailgun`. Такођер дефинишемо посебан транспорт који користимо специфично за `@yahoo.com` е-поруке. У појединим сценаријима је пожељно користити специфичног провајдера или IP адресу отправљача за одређени домен ради побољшања испоруке. Ово је опционално.

### DocumentDB

Када се повезујете на `DocumentDB` желећете да наведете `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` како бисте били компатибилни са подразумеваним поставкама.

---