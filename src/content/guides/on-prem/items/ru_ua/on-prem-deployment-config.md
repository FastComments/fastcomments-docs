FastComments использует переменные окружения для конфигурации. В следующем списке перечислены все поддерживаемые переменные, имеющие отношение к On-Prem.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Тип окружения.                                                                                                                                     | Да       | production, dev                                       |
| MONGO_URI                      |                             | URI подключения к БД.                                                                                                                              | Да       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Включает использование SSL для подключения к базе данных.                                                                                         | Нет      | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Включает проверку сертификата против CA при подключении к Mongo.                                                                                   | Нет      | true, false                                           |
| MONGO_SSL_CA                   |                             | PEM-файл CA для Mongo SSL.                                                                                                                         | Нет      | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Электронная почта, на которую должны приходить важные системные уведомления.                                                                      | Нет      | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Соль для хеширования IP-адресов.                                                                                                                   | Да       |                                                       |
| SESSION_SECRET                 |                             | Ключ, используемый для подписи сессий.                                                                                                             | Да       |                                                       |
| SESSION_STORE_SECRET           |                             | Ключ, используемый для подписи/хеширования сессий в хранилище. Должен отличаться от SESSION_SECRET.                                                | Да       |                                                       |
| HOSTNAME                       |                             | Хостнейм, где развернут FastComments (панель администрирования и т.д.). НЕ должен включать порт или протокол.                                      | Да       | example.com                                           |
| HOST_ADDR                      |                             | Доступный URI, где развернут FastComments (панель администрирования и т.д.).                                                                      | Да       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Путь в локальной файловой системе к конфигурации электронной почты (SMTP, сопоставления доменов/провайдеров и т.д.).                                 | Да       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Заголовок "From Name" в письме.                                                                                                                    | Нет      | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Логотип в футере письма.                                                                                                                           | Нет      | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Переопределение для "defaultTransport" в EMAIL_CONFIG_PATH. Полезно при развертывании одного и того же файла конфигурации в разных окружениях.     | Нет      | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID вашей учетной записи на fastcomments.com. Используется для регистрации лицензионного ключа.                                                     | Нет      |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Лицензионный ключ для on-prem.                                                                                                                     | Нет      |                                                       |
| GIPHY_API_KEY                  |                             | Ключ Giphy API. Если не указан, следует создать правило конфигурации, которое отключает селектор GIF.                                              | Нет      |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Используется для интеграции giphy. Также может быть переопределено правилами кастомизации виджета.                                                  | Нет      | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Используется для функций на базе openai, таких как опциональное обнаружение спама с помощью GPT.                                                   | Нет      |                                                       |
| CDN_HOST_ADDR                  |                             | Хостнейм, откуда будут загружаться ассеты. По умолчанию принимает значение HOSTNAME.                                                              | Нет      | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Хостнейм, откуда будут загружаться большие файлы (например, экспорты). По умолчанию принимает значение CDN_HOST_ADDR.                              | Нет      | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Где должны храниться большие файлы, такие как экспорты.                                                                                            | Нет      | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Хостнейм, откуда должны отправляться письма.                                                                                                       | Нет      | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Имя cookie fastcomments.                                                                                                                           | Нет      |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Значение поля "hostname" cookie. Рекомендуется добавлять префикс с точкой.                                                                         | Нет      | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Используется для загрузок файлов пользователями, аватаров и т.д. По умолчанию — локальная FS, если не определено.                                  | Нет      |                                                       |
| S3_SECRET_KEY                  |                             | Используется для загрузок файлов пользователями, аватаров и т.д.                                                                                   | Нет      |                                                       |
| S3_REGION                      |                             | Используется для загрузок файлов пользователями, аватаров и т.д.                                                                                   | Нет      |                                                       |
| S3_BUCKET                      |                             | Используется для загрузок файлов пользователями, аватаров и т.д.                                                                                   | Нет      |                                                       |
| S3_HOST                        |                             | Используется для загрузок файлов пользователями, аватаров и т.д.                                                                                   | Нет      |                                                       |
| CACHE_DIR                      |                             | Место для хранения опционального офлайн-кэша на случай недоступности БД. Периодически обновляется с топ-100 ветками комментариев.                    | Нет      |                                                       |
| BACKUP_DIR                     |                             | Место для хранения данных на случай недоступности БД. Если комментарий отправлен при недоступной БД, он попадает сюда и обрабатывается позже.       | Нет      |                                                       |

Обратите внимание, что все переменные, связанные с доменами, используют постфикс `_HOST` или `_ADDR`. Разница:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` должен содержать путь к JSON-файлу со следующим примерным форматом:

[inline-code-attrs-start title = 'Конфигурация электронной почты'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

В приведенном выше примере мы определяем транспорт по умолчанию для электронной почты `SMTP` с именем `mailgun`. Также мы определяем специальный транспорт, который используем специально для писем `@yahoo.com`. В некоторых сценариях бывает желательно использовать конкретного провайдера или IP-адрес отправителя для домена, чтобы настроить доставляемость. Это необязательно.

### DocumentDB

При подключении к `DocumentDB` рекомендуется указать `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`, чтобы быть совместимым со стандартными настройками.

---