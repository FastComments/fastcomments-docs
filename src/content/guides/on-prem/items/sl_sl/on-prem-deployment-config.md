FastComments uporablja okoljske spremenljivke za konfiguracijo. Naslednji seznam opisuje vse podprte spremenljivke, pomembne za On-Prem.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Vrsta okolja.                                                                                                                                      | Da       | production, dev                                       |
| MONGO_URI                      |                             | URI za povezavo s podatkovno bazo.                                                                                                                 | Da       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Omogoči uporabo SSL za povezavo s podatkovno bazo.                                                                                                 | Ne       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Omogoči preverjanje potrdila proti CA pri povezovanju z Mongo.                                                                                      | Ne       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem datoteka.                                                                                                                          | Ne       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-pošta, kamor naj gredo pomembna obvestila povezana s sistemom.                                                                                   | Ne       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Sol za zgoščevanje IP naslovov.                                                                                                                    | Da       |                                                       |
| SESSION_SECRET                 |                             | Ključ, uporabljen za podpisovanje sej.                                                                                                             | Da       |                                                       |
| SESSION_STORE_SECRET           |                             | Ključ, uporabljen za podpisovanje/zgoščevanje sej v shrambi. Mora biti drugačen od SESSION_SECRET.                                                 | Da       |                                                       |
| HOSTNAME                       |                             | Gostiteljsko ime, kjer je nameščen FastComments (npr. administrativni vmesnik). NE sme vsebovati porta ali protokola.                              | Da       | example.com                                           |
| HOST_ADDR                      |                             | Dostopni URI, kjer je nameščen FastComments (npr. administrativni vmesnik).                                                                        | Da       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Pot v lokalnem datotečnem sistemu, kjer se nahaja konfiguracija e-pošte (SMTP, preslikave domen/ponudnikov itd.).                                   | Da       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Glava e-pošte "From Name".                                                                                                                          | Ne       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logotip v nogi e-pošte.                                                                                                                             | Ne       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Prepis za "defaultTransport" v EMAIL_CONFIG_PATH. Uporabno pri nameščanju iste konfiguracijske datoteke v različnih okoljih.                         | Ne       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID vašega računa na fastcomments.com. Uporablja se za registracijo vašega licenčnega ključa.                                                       | Ne       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-prem licenčni ključ.                                                                                                                             | Ne       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API ključ. Če ni določen, ustvarite konfiguracijsko pravilo, ki onemogoči izbiro GIF-ov.                                                     | Ne       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Uporablja se za integracijo Giphy. Lahko ga tudi prepišejo pravila za prilagoditev widgeta.                                                         | Ne       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Uporablja se za funkcije, podprte z OpenAI, kot je izbirno GPT-podprto zaznavanje neželene pošte.                                                  | Ne       |                                                       |
| CDN_HOST_ADDR                  |                             | Gostiteljsko ime, od koder se bodo prenašale datoteke z viri (assets). Privzeto je vrednost HOSTNAME.                                              | Ne       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Gostiteljsko ime, od koder se prenesejo velike datoteke (npr. izvozi). Privzeto vrednost jemlje iz CDN_HOST_ADDR.                                   | Ne       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Kje naj bodo shranjene velike datoteke, kot so izvozi.                                                                                              | Ne       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Gostiteljsko ime, iz katerega naj bodo e-pošte poslane.                                                                                            | Ne       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Ime piškotka fastcomments.                                                                                                                          | Ne       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Vrednost polja "hostname" v piškotku. Priporočljivo je predpono s piko.                                                                             | Ne       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Uporablja se za nalaganje datotek uporabnikov, avatarje itd. Privzeto je lokalni datotečni sistem, če ni definirano.                                | Ne       |                                                       |
| S3_SECRET_KEY                  |                             | Uporablja se za nalaganje datotek uporabnikov, avatarje itd.                                                                                       | Ne       |                                                       |
| S3_REGION                      |                             | Uporablja se za nalaganje datotek uporabnikov, avatarje itd.                                                                                       | Ne       |                                                       |
| S3_BUCKET                      |                             | Uporablja se za nalaganje datotek uporabnikov, avatarje itd.                                                                                       | Ne       |                                                       |
| S3_HOST                        |                             | Uporablja se za nalaganje datotek uporabnikov, avatarje itd.                                                                                       | Ne       |                                                       |
| CACHE_DIR                      |                             | Lokacija za shranjevanje opcijskega brez povezave predpomnilnika, za primer, ko podatkovna baza ni na voljo. Periodično osveževan z 100 najboljšimi nitmi komentarjev. | Ne       |                                                       |
| BACKUP_DIR                     |                             | Lokacija za shranjevanje podatkov za primer, ko podatkovna baza ni na voljo. Če je komentar poslan, ko DB ni na voljo, gre sem in se obdela kasneje. | Ne       |                                                       |

Upoštevajte, da vse spremenljivke povezane z domenami uporabljajo pripono `_HOST` ali `_ADDR`. Razlika je:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` naj vsebuje pot do JSON datoteke z naslednjo vzorčno obliko:

[inline-code-attrs-start title = 'Konfiguracija e-pošte'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

V zgornjem primeru smo definirali privzeti SMTP transport z imenom `mailgun`. Prav tako smo definirali poseben transport, ki ga uporabljamo posebej za e-pošto `@yahoo.com`. V nekaterih scenarijih je smiselno uporabiti določenega ponudnika ali pošiljajoči IP za domeno za nastavitev dostave. To je neobvezno.

### DocumentDB

Ko se povezujete na `DocumentDB`, nastavite `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem`, da boste združljivi z privzetimi nastavitvami.

---