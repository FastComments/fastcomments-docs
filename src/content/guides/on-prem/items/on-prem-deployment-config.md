FastComments utilizes environment variables for configuration. The following list outlines all supported variables that are relevant to On-Prem.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Environment type.                                                                                                                                  | Yes      | production, dev                                       |
| MONGO_URI                      |                             | DB Connection URI.                                                                                                                                 | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Enables using SSL to connect to the database.                                                                                                      | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Enables validating the cert against the CA when connecting to Mongo.                                                                               | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem file.                                                                                                                             | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email where important system-related notifications should go.                                                                                      | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt for hashing IP addresses.                                                                                                                     | Yes      |                                                       |
| SESSION_SECRET                 |                             | The key used to sign sessions.                                                                                                                     | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | The key used to sign/hash sessions in storage. Must be different than SESSION_SECRET.                                                              | Yes      |                                                       |
| HOSTNAME                       |                             | The hostname where FastComments is deployed (admin dashboard etc). Should NOT include port or protocol.                                            | Yes      | example.com                                           |
| HOST_ADDR                      |                             | An accessible URI where FastComments is deployed (admin dashboard etc).                                                                            | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | A path on the local file system where the email config (SMTP, domain/provider mappings, etc) is.                                                   | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Email "From Name" header.                                                                                                                          | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Email footer logo.                                                                                                                                 | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Override for "defaultTransport" in EMAIL_CONFIG_PATH. Useful for deploying same config file to different envs.                                     | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | The ID of your account on fastcomments.com. Used to register your license key.                                                                     | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | An on-prem license key.                                                                                                                            | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Key. If not specified, you should create a config rule that disables the gif picker.                                                     | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Used for giphy integration. Can also be overridden with widget customization rules.                                                                | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Used for openai powered features like optional GPT-based spam detection.                                                                           | No       |                                                       |
| CDN_HOST_ADDR                  |                             | The hostname where assets will be fetched from. Defaults to value of HOSTNAME.                                                                     | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | The hostname where large files (like exports) are fetched from. Defaults to value of CDN_HOST_ADDR.                                                | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Where large files, like exports, should be stored.                                                                                                 | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | The hostname where emails should be sent from.                                                                                                     | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | The name of the fastcomments cookie.                                                                                                               | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | The value of the cookie "hostname" field. Recommended to prefix with dot.                                                                          | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Used for user file uploads, avatars, etc. Defaults to local FS if undefined.                                                                       | No       |                                                       |
| S3_SECRET_KEY                  |                             | Used for user file uploads, avatars, etc.                                                                                                          | No       |                                                       |
| S3_REGION                      |                             | Used for user file uploads, avatars, etc.                                                                                                          | No       |                                                       |
| S3_BUCKET                      |                             | Used for user file uploads, avatars, etc.                                                                                                          | No       |                                                       |
| S3_HOST                        |                             | Used for user file uploads, avatars, etc.                                                                                                          | No       |                                                       |
| CACHE_DIR                      |                             | Location to store optional offline cache, for when DB is not available. Periodically refreshed with top 100 comment threads.                       | No       |                                                       |
| BACKUP_DIR                     |                             | Location to store data for when DB is not available. If a comment is submitted when the DB is not available, it goes here, and is processed later. | No       |                                                       |

Note that all domain-related variables use the `_HOST` or `_ADDR` postfix. The difference is:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

The `EMAIL_CONFIG_PATH` should contain a path to a JSON file with the following example format:

inline-code-attrs-start title = 'Email Config'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

In the above example we define a default `SMTP` email transport called `mailgun`. We also define special transport that we use specifically for
`@yahoo.com` emails. In some scenarios it is desirable to use a specific provider or sending IP for a domain to tune delivery. This is optional.

### DocumentDB

When connecting to `DocumentDB` you will want to specify `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` to be compatible with default settings.
