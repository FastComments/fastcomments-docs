FastComments използва променливи на средата за конфигурация. Следният списък описва всички поддържани променливи, които са релевантни за On-Prem.

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Тип на средата.                                                                                                                                     | Да       | production, dev                                       |
| MONGO_URI                      |                             | URI за връзка към базата данни.                                                                                                                    | Да       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Позволява използването на SSL за връзка с базата данни.                                                                                             | Не       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Позволява валидиране на сертификата спрямо CA при свързване към Mongo.                                                                              | Не       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem файл.                                                                                                                             | Не       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Имейл, на който да се изпращат важни системни известия.                                                                                            | Не       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Салт за хеширане на IP адреси.                                                                                                                     | Да       |                                                       |
| SESSION_SECRET                 |                             | Ключът, използван за подписване на сесиите.                                                                                                        | Да       |                                                       |
| SESSION_STORE_SECRET           |                             | Ключът, използван за подписване/хеширане на сесиите в сториджа. Трябва да е различен от SESSION_SECRET.                                             | Да       |                                                       |
| HOSTNAME                       |                             | Името на хоста, където е внедрен FastComments (административен панел и т.н.). НЕ трябва да включва порт или протокол.                                | Да       | example.com                                           |
| HOST_ADDR                      |                             | Достъпен URI, където е внедрен FastComments (административен панел и т.н.).                                                                         | Да       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Път в локалната файлова система, където се намира конфигурацията за имейл (SMTP, съпоставяне на домейни/доставчици и т.н.).                          | Да       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Заглавие "From Name" в имейлите.                                                                                                                    | Не       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Лого във футъра на имейла.                                                                                                                         | Не       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Презапис за "defaultTransport" в EMAIL_CONFIG_PATH. Полезно при внедряване на един и същ конфигурационен файл в различни среди.                        | Не       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID на вашия акаунт в fastcomments.com. Използва се за регистриране на лицензионния ключ.                                                            | Не       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Лицензен ключ за on-prem инсталация.                                                                                                               | Не       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API ключ. Ако не е зададено, трябва да създадете правило за конфигурация, което да деактивира избора на GIF.                                   | Не       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Използва се за интеграцията с Giphy. Може да бъде презаписано и чрез правила за персонализиране на уиджета.                                         | Не       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Използва се за функции, базирани на OpenAI, като опционално GPT-базирано откриване на спам.                                                         | Не       |                                                       |
| CDN_HOST_ADDR                  |                             | Името на хоста, откъдето ще се зареждат ресурсите. По подразбиране е стойността на HOSTNAME.                                                       | Не       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Името на хоста, откъдето се зареждат големи файлове (например експорти). По подразбиране е стойността на CDN_HOST_ADDR.                                | Не       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Къде трябва да се съхраняват големи файлове, като експорти.                                                                                         | Не       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Името на хоста, от който трябва да се изпращат имейлите.                                                                                            | Не       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Името на бисквитката на fastcomments.                                                                                                              | Не       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Стойността на полето "hostname" в бисквитката. Препоръчва се да се сложи префикс точка.                                                             | Не       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Използва се за качвания на файлове от потребители, аватари и т.н. По подразбиране е локална файлова система, ако не е дефинирано.                      | Не       |                                                       |
| S3_SECRET_KEY                  |                             | Използва се за качвания на файлове от потребители, аватари и т.н.                                                                                  | Не       |                                                       |
| S3_REGION                      |                             | Използва се за качвания на файлове от потребители, аватари и т.н.                                                                                  | Не       |                                                       |
| S3_BUCKET                      |                             | Използва се за качвания на файлове от потребители, аватари и т.н.                                                                                  | Не       |                                                       |
| S3_HOST                        |                             | Използва се за качвания на файлове от потребители, аватари и т.н.                                                                                  | Не       |                                                       |
| CACHE_DIR                      |                             | Местоположение за съхранение на опционален офлайн кеш, за случаи когато БД не е налична. Периодично се обновява с топ 100 нишки с коментари.           | Не       |                                                       |
| BACKUP_DIR                     |                             | Местоположение за съхранение на данни за случаи когато БД не е налична. Ако коментар бъде изпратен, когато БД не е достъпна, отива тук и се обработва по-късно. | Не       |                                                       |

Обърнете внимание, че всички променливи, свързани с домейни, използват постфиксите `_HOST` или `_ADDR`. Разликата е:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Пътят в `EMAIL_CONFIG_PATH` трябва да сочи към JSON файл със следния примерен формат:

[inline-code-attrs-start title = 'Конфигурация на имейл'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

В горния пример дефинираме по подразбиране `SMTP` имейл транспорт, наречен `mailgun`. Също така дефинираме специален транспорт, който използваме конкретно за имейли от `@yahoo.com`. В някои сценарии е желателно да се използва специфичен доставчик или IP за изпращане за даден домейн, за да се подобри доставяемостта. Това е опционално.

### DocumentDB

При свързване към `DocumentDB` е препоръчително да зададете `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`, за да сте съвместими с настройките по подразбиране.