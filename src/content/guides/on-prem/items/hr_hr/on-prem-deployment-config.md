FastComments koristi varijable okoline za konfiguraciju. Sljedeći popis navodi sve podržane varijable koje su relevantne za On-Prem.


| Varijabla                      | Zadano                      | Info                                                                                                                                               | Obavezno | Primjeri ili valjane vrijednosti                         |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|---------------------------------------------------------|
| NODE_ENV                       |                             | Tip okoline.                                                                                                                                       | Da       | production, dev                                         |
| MONGO_URI                      |                             | URI za povezivanje s bazom podataka.                                                                                                               | Da       |                                                         |
| MONGO_ENABLE_SSL               | false                       | Omogućuje korištenje SSL-a za povezivanje s bazom podataka.                                                                                        | Ne       | true, false                                             |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Omogućuje provjeru certifikata protiv CA pri povezivanju na Mongo.                                                                                  | Ne       | true, false                                             |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem datoteka.                                                                                                                          | Ne       | /path/to/some-cert.pem                                  |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-pošta na koju bi trebale stizati važne obavijesti vezane uz sustav.                                                                               | Ne       | admin-group@bigcorp.com                                 |
| IP_HASH_SALT                   |                             | Salt (sol) za hashiranje IP adresa.                                                                                                                 | Da       |                                                         |
| SESSION_SECRET                 |                             | Ključ koji se koristi za potpisivanje sesija.                                                                                                       | Da       |                                                         |
| SESSION_STORE_SECRET           |                             | Ključ koji se koristi za potpisivanje/hashiranje sesija u spremištu. Mora biti različit od SESSION_SECRET.                                          | Da       |                                                         |
| HOSTNAME                       |                             | Hostname na kojem je FastComments postavljen (administratorska konzola itd.). NE smije uključivati port ili protokol.                                 | Da       | example.com                                             |
| HOST_ADDR                      |                             | Dostupan URI na kojem je FastComments postavljen (administratorska konzola itd.).                                                                   | Da       | https://example.com                                     |
| EMAIL_CONFIG_PATH              |                             | Putanja na lokalnom datotečnom sustavu gdje se nalazi konfiguracija e-pošte (SMTP, mapiranja domena/ponuđača itd.).                                  | Da       | /my/config.json                                         |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Polje "From Name" u zaglavlju e-pošte.                                                                                                              | Ne       | My Company Name                                         |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo u podnožju e-pošte.                                                                                                                            | Ne       | https://exmaple.com/footer.png                          |
| EMAIL_DEFAULT_TRANSPORT        |                             | Override za "defaultTransport" u EMAIL_CONFIG_PATH. Korisno za raspoređivanje iste datoteke konfiguracije u različita okruženja.                     | Ne       | myTransportName                                         |
| ON_PREM_TENANT_ID              |                             | ID vašeg računa na fastcomments.com. Koristi se za registraciju vašeg ključa licence.                                                               | Ne       |                                                         |
| ON_PREM_LICENSE_KEY            |                             | Licencni ključ za on-prem.                                                                                                                          | Ne       |                                                         |
| GIPHY_API_KEY                  |                             | Giphy API ključ. Ako nije naveden, trebate stvoriti pravilo konfiguracije koje će onemogućiti odabir gifova.                                         | Ne       |                                                         |
| GIPHY_DEFAULT_RATING           | pg                          | Koristi se za integraciju s Giphyjem. Može se također nadjačati pravilima prilagodbe widgeta.                                                       | Ne       | g, pg, pg-13, r                                         |
| OPENAI_SECRET_KEY              |                             | Koristi se za značajke pokretane OpenAI-jem, poput opcionalne GPT-bazirane detekcije spama.                                                          | Ne       |                                                         |
| CDN_HOST_ADDR                  |                             | Hostname s kojeg će se dohvaćati resursi. Zadano je vrijednost HOSTNAME.                                                                            | Ne       | example.com                                             |
| LARGE_FILE_HOST_ADDR           |                             | Hostname s kojeg se dohvaćaju velike datoteke (npr. izvozi). Zadano je vrijednost CDN_HOST_ADDR.                                                    | Ne       | example.com                                             |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Gdje bi se trebale pohranjivati velike datoteke, poput izvoza.                                                                                      | Ne       | local_disk, s3                                          |
| FROM_EMAIL_HOST                |                             | Hostname s kojeg bi se trebale slati e-pošte.                                                                                                       | Ne       | example.com                                             |
| COOKIE_ID                      | fastcomments.sid            | Naziv fastcomments kolačića.                                                                                                                         | Ne       |                                                         |
| COOKIE_HOSTNAME                | .fastcomments.com           | Vrijednost polja "hostname" u kolačiću. Preporučuje se prefiksati točkom.                                                                           | Ne       | .example.com                                            |
| S3_ACCESS_KEY                  |                             | Koristi se za korisnička učitavanja datoteka, avatare itd. Zadano je lokalni datotečni sustav ako nije definirano.                                    | Ne       |                                                         |
| S3_SECRET_KEY                  |                             | Koristi se za korisnička učitavanja datoteka, avatare itd.                                                                                            | Ne       |                                                         |
| S3_REGION                      |                             | Koristi se za korisnička učitavanja datoteka, avatare itd.                                                                                            | Ne       |                                                         |
| S3_BUCKET                      |                             | Koristi se za korisnička učitavanja datoteka, avatare itd.                                                                                            | Ne       |                                                         |
| S3_HOST                        |                             | Koristi se za korisnička učitavanja datoteka, avatare itd.                                                                                            | Ne       |                                                         |
| CACHE_DIR                      |                             | Lokacija za pohranu opcionalne offline cache (predmemorije), za slučajeve kada DB nije dostupan. Periodično se osvježava s top 100 niti komentara.    | Ne       |                                                         |
| BACKUP_DIR                     |                             | Lokacija za pohranu podataka za slučajeve kada DB nije dostupan. Ako se komentar pošalje kada DB nije dostupan, ide ovdje i obrađuje se kasnije.     | Ne       |                                                         |

Imajte na umu da sve varijable vezane uz domenu koriste sufiks `_HOST` ili `_ADDR`. Razlika je:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` bi trebao sadržavati putanju do JSON datoteke sa sljedećim primjernim formatom:

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

U gornjem primjeru definiramo zadani `SMTP` transport e-pošte nazvan `mailgun`. Također definiramo poseban transport koji koristimo posebno za e-poruke `@yahoo.com`. U nekim scenarijima poželjno je koristiti određenog pružatelja usluge ili IP adresu za slanje za određenu domenu kako bi se poboljšala isporuka. Ovo je opcionalno.

### DocumentDB

Prilikom povezivanja na `DocumentDB` trebate navesti `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` kako biste bili kompatibilni s zadanim postavkama.