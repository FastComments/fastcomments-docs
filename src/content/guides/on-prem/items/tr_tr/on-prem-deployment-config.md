FastComments, yapılandırma için çevre değişkenlerini (environment variables) kullanır. Aşağıdaki liste, On-Prem ile ilgili tüm desteklenen değişkenleri özetler.

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Ortam türü.                                                                                                                                        | Evet     | production, dev                                       |
| MONGO_URI                      |                             | DB Bağlantı URI'si.                                                                                                                               | Evet     |                                                       |
| MONGO_ENABLE_SSL               | false                       | Veritabanına bağlanmak için SSL kullanmayı etkinleştirir.                                                                                         | Hayır    | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Mongo'ya bağlanırken sertifikayı CA'ya karşı doğrulamayı etkinleştirir.                                                                           | Hayır    | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem dosyası.                                                                                                                          | Hayır    | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Önemli sistemle ilgili bildirimlerin gönderileceği e-posta adresi.                                                                                | Hayır    | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | IP adreslerini hashlemek için salt değeri.                                                                                                        | Evet     |                                                       |
| SESSION_SECRET                 |                             | Oturumları imzalamak için kullanılan anahtar.                                                                                                     | Evet     |                                                       |
| SESSION_STORE_SECRET           |                             | Depolamada oturumları imzalamak/hashlemek için kullanılan anahtar. SESSION_SECRET'ten farklı olmalıdır.                                           | Evet     |                                                       |
| HOSTNAME                       |                             | FastComments'ın dağıtıldığı ana bilgisayar adı (admin paneli vb.). Port veya protokol içermemelidir.                                               | Evet     | example.com                                           |
| HOST_ADDR                      |                             | FastComments'ın erişilebilir URI'si (admin paneli vb.).                                                                                           | Evet     | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | E-posta yapılandırmasının (SMTP, domain/sağlayıcı eşlemeleri vb.) bulunduğu yerel dosya sistemi yolu.                                              | Evet     | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | E-posta "Gönderen Adı" (From Name) başlığı.                                                                                                       | Hayır    | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | E-posta footer logosu.                                                                                                                             | Hayır    | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | EMAIL_CONFIG_PATH içindeki "defaultTransport" için override. Aynı yapılandırma dosyasını farklı ortamlara dağıtırken faydalıdır.                    | Hayır    | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | fastcomments.com üzerindeki hesabınızın ID'si. Lisans anahtarınızı kaydetmek için kullanılır.                                                     | Hayır    |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-prem lisans anahtarı.                                                                                                                           | Hayır    |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Anahtarı. Belirtilmemişse, gif seçiciyi devre dışı bırakan bir yapılandırma kuralı oluşturmalısınız.                                     | Hayır    |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Giphy entegrasyonu için kullanılır. Widget özelleştirme kurallarıyla da override edilebilir.                                                      | Hayır    | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | GPT tabanlı isteğe bağlı spam tespiti gibi OpenAI destekli özellikler için kullanılır.                                                              | Hayır    |                                                       |
| CDN_HOST_ADDR                  |                             | Varlıkların (assets) çekileceği ana bilgisayar adı. Varsayılan olarak HOSTNAME değerini kullanır.                                                  | Hayır    | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Büyük dosyaların (ör. exportlar) çekileceği ana bilgisayar adı. Varsayılan olarak CDN_HOST_ADDR değerini kullanır.                                 | Hayır    | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Exportlar gibi büyük dosyaların nerede saklanacağı.                                                                                               | Hayır    | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | E-postaların gönderileceği ana bilgisayar adı.                                                                                                     | Hayır    | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | fastcomments çerezinin (cookie) adı.                                                                                                               | Hayır    |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Çerezin "hostname" alanının değeri. Önek olarak nokta kullanılması önerilir.                                                                      | Hayır    | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Kullanıcı dosya yüklemeleri, avatarlar vb. için kullanılır. Tanımsızsa varsayılan yerel dosya sistemi kullanılır.                                   | Hayır    |                                                       |
| S3_SECRET_KEY                  |                             | Kullanıcı dosya yüklemeleri, avatarlar vb. için kullanılır.                                                                                        | Hayır    |                                                       |
| S3_REGION                      |                             | Kullanıcı dosya yüklemeleri, avatarlar vb. için kullanılır.                                                                                        | Hayır    |                                                       |
| S3_BUCKET                      |                             | Kullanıcı dosya yüklemeleri, avatarlar vb. için kullanılır.                                                                                        | Hayır    |                                                       |
| S3_HOST                        |                             | Kullanıcı dosya yüklemeleri, avatarlar vb. için kullanılır.                                                                                        | Hayır    |                                                       |
| CACHE_DIR                      |                             | DB kullanılamadığında isteğe bağlı çevrimdışı önbelleği (offline cache) depolamak için konum. Periyodik olarak en popüler 100 yorum dizisi ile yenilenir. | Hayır    |                                                       |
| BACKUP_DIR                     |                             | DB kullanılamadığında verileri saklamak için konum. DB kullanılamadığında gönderilen bir yorum buraya gider ve daha sonra işlenir.                  | Hayır    |                                                       |

Tüm alan adı (domain) ile ilgili değişkenlerin `_HOST` veya `_ADDR` son ekini (postfix) kullandığını unutmayın. Fark şu şekildedir:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH`, aşağıdaki örnek formatına sahip bir JSON dosyasına giden bir yol içermelidir:

[inline-code-attrs-start title = 'E-posta Yapılandırması'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Yukarıdaki örnekte, `mailgun` adında varsayılan bir `SMTP` e-posta transportu tanımlıyoruz. Ayrıca özellikle `@yahoo.com` e-postaları için kullandığımız özel bir transport tanımlıyoruz. Bazı senaryolarda teslimatı ayarlamak için bir alan adı için belirli bir sağlayıcıya veya gönderim IP'sine (sending IP) özel olarak yönlendirme yapmak istenebilir. Bu isteğe bağlıdır.

### DocumentDB

`DocumentDB`'ye bağlanırken, varsayılan ayarlarla uyumlu olması için `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` belirtmek istersiniz.