FastComments कॉन्फ़िगरेशन के लिए environment variables का उपयोग करता है। निम्नलिखित सूची उन सभी समर्थित variables को रेखांकित करती है जो On-Prem के लिए प्रासंगिक हैं।


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Environment का प्रकार।                                                                                                                                  | Yes      | production, dev                                       |
| MONGO_URI                      |                             | DB Connection URI।                                                                                                                                 | Yes      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Database से connect करने के लिए SSL का उपयोग सक्षम करता है।                                                                                                      | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongo से connect करते समय CA के विरुद्ध cert को validate करना सक्षम करता है।                                                                               | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem फ़ाइल।                                                                                                                             | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email जहाँ महत्वपूर्ण system-संबंधित notifications जाने चाहिए।                                                                                      | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IP addresses को hash करने के लिए salt।                                                                                                                     | Yes      |                                                       |
| SESSION_SECRET                 |                             | Sessions को sign करने के लिए उपयोग की जाने वाली key।                                                                                                                     | Yes      |                                                       |
| SESSION_STORE_SECRET           |                             | Storage में sessions को sign/hash करने के लिए उपयोग की जाने वाली key। यह SESSION_SECRET से अलग होनी चाहिए।                                                              | Yes      |                                                       |
| HOSTNAME                       |                             | Hostname जहाँ FastComments deploy किया गया है (admin dashboard आदि)। इसमें port या protocol शामिल नहीं होना चाहिए।                                            | Yes      | example.com                                           |
| HOST_ADDR                      |                             | एक accessible URI जहाँ FastComments deploy किया गया है (admin dashboard आदि)।                                                                            | Yes      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Local file system पर path जहाँ email config (SMTP, domain/provider mappings, आदि) है।                                                   | Yes      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Email "From Name" header।                                                                                                                          | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Email footer logo।                                                                                                                                 | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH में "defaultTransport" के लिए override। एक ही config फ़ाइल को अलग-अलग envs में deploy करने के लिए उपयोगी।                                     | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com पर आपके account की ID। आपकी license key register करने के लिए उपयोग किया जाता है।                                                                     | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | एक on-prem license key।                                                                                                                            | No       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Key। यदि निर्दिष्ट नहीं है, तो आपको एक ऐसा config rule बनाना चाहिए जो gif picker को disable कर दे।                                                     | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Giphy integration के लिए उपयोग किया जाता है। Widget customization rules से भी override किया जा सकता है।                                                                | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Openai द्वारा संचालित features के लिए उपयोग किया जाता है, जैसे optional GPT-आधारित spam detection।                                                                           | No       |                                                       |
| CDN_HOST_ADDR                  |                             | Hostname जहाँ से assets fetch किए जाएँगे। Default रूप से HOSTNAME का मान।                                                                     | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Hostname जहाँ से बड़ी फ़ाइलें (जैसे exports) fetch की जाती हैं। Default रूप से CDN_HOST_ADDR का मान।                                                | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | बड़ी फ़ाइलें, जैसे exports, कहाँ store की जानी चाहिए।                                                                                                 | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Hostname जहाँ से emails भेजे जाने चाहिए।                                                                                                     | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Fastcomments cookie का नाम।                                                                 | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Cookie के "hostname" field का मान। Dot से prefix करने की अनुशंसा की जाती है।                                                                          | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | User file uploads, avatars, आदि के लिए उपयोग किया जाता है। Undefined होने पर default रूप से local FS।                                                                       | No       |                                                       |
| S3_SECRET_KEY                  |                             | User file uploads, avatars, आदि के लिए उपयोग किया जाता है।                                                                                                          | No       |                                                       |
| S3_REGION                      |                             | User file uploads, avatars, आदि के लिए उपयोग किया जाता है।                                                                                                          | No       |                                                       |
| S3_BUCKET                      |                             | User file uploads, avatars, आदि के लिए उपयोग किया जाता है।                                                                                                          | No       |                                                       |
| S3_HOST                        |                             | User file uploads, avatars, आदि के लिए उपयोग किया जाता है।                                                                                                          | No       |                                                       |
| CACHE_DIR                      |                             | Optional offline cache store करने का स्थान, जब DB उपलब्ध नहीं हो। Top 100 comment threads के साथ समय-समय पर refresh किया जाता है।                       | No       |                                                       |
| BACKUP_DIR                     |                             | Data store करने का स्थान जब DB उपलब्ध नहीं हो। यदि DB उपलब्ध नहीं होने पर कोई comment submit किया जाता है, तो यह यहाँ जाता है और बाद में process किया जाता है। | No       |                                                       |

ध्यान दें कि सभी domain-संबंधित variables `_HOST` या `_ADDR` postfix का उपयोग करते हैं। अंतर यह है:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` में निम्नलिखित उदाहरण format के साथ एक JSON फ़ाइल का path होना चाहिए:

[inline-code-attrs-start title = 'Email Config'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

ऊपर के उदाहरण में हम `mailgun` नामक एक default `SMTP` email transport define करते हैं। हम एक special transport भी define करते हैं जिसका उपयोग हम विशेष रूप से
`@yahoo.com` emails के लिए करते हैं। कुछ scenarios में किसी domain के लिए delivery को tune करने हेतु किसी specific provider या sending IP का उपयोग करना वांछनीय होता है। यह optional है।

### DocumentDB

`DocumentDB` से connect करते समय आप default settings के साथ compatible होने के लिए `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` निर्दिष्ट करना चाहेंगे।