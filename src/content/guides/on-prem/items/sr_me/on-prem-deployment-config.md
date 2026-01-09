FastComments koristi environment varijable za konfiguraciju. Slijedeći spisak navodi sve podržane varijable koje su relevantne za On-Prem.


| Variable                       | Podrazumijevano            | Opis                                                                                                                                               | Obavezno | Primjeri ili važeće vrijednosti                              |
|--------------------------------|----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------------|
| NODE_ENV                       |                            | Tip okruženja.                                                                                                                                     | Da       | production, dev                                             |
| MONGO_URI                      |                            | URI za povezivanje sa bazom podataka.                                                                                                             | Da       |                                                             |
| MONGO_ENABLE_SSL               | false                      | Omogućava korišćenje SSL-a za povezivanje sa bazom podataka.                                                                                       | Ne       | true, false                                                 |
| MONGO_ENABLE_SSL_VALIDATE      | false                      | Omogućava validaciju sertifikata prema CA prilikom povezivanja na Mongo.                                                                          | Ne       | true, false                                                 |
| MONGO_SSL_CA                   |                            | Mongo SSL CA pem fajl.                                                                                                                             | Ne       | /path/to/some-cert.pem                                      |
| ADMIN_NOTIFICATIONS_EMAIL      |                            | Email na koji treba da stižu važne sistemske notifikacije.                                                                                        | Ne       | admin-group@bigcorp.com                                     |
| IP_HASH_SALT                   |                            | Sol za heširanje IP adresa.                                                                                                                        | Da       |                                                             |
| SESSION_SECRET                 |                            | Ključ koji se koristi za potpisivanje sesija.                                                                                                     | Da       |                                                             |
| SESSION_STORE_SECRET           |                            | Ključ koji se koristi za potpisivanje/heširanje sesija u skladištu. Mora biti drugačiji od SESSION_SECRET.                                        | Da       |                                                             |
| HOSTNAME                       |                            | Hostname na kojem je FastComments postavljen (admin dashboard itd). Ne smije uključivati port ili protokol.                                        | Da       | example.com                                                 |
| HOST_ADDR                      |                            | Pristupačni URI na kojem je FastComments dostupan (admin dashboard itd).                                                                           | Da       | https://example.com                                         |
| EMAIL_CONFIG_PATH              |                            | Putanja na lokalnom fajl sistemu gdje se nalazi konfiguracija e-pošte (SMTP, mapiranja domena/ provajdera, itd).                                   | Da       | /my/config.json                                             |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot         | Zaglavlje "From Name" u emailu.                                                                                                                    | Ne       | My Company Name                                             |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png| Logo u podnožju emaila.                                                                                                                            | Ne       | https://exmaple.com/footer.png                              |
| EMAIL_DEFAULT_TRANSPORT        |                            | Override za "defaultTransport" u EMAIL_CONFIG_PATH. Korisno za deploy istog konfiguracionog fajla u različita okruženja.                            | Ne       | myTransportName                                             |
| ON_PREM_TENANT_ID              |                            | ID vašeg naloga na fastcomments.com. Koristi se za registraciju vašeg licence ključa.                                                              | Ne       |                                                             |
| ON_PREM_LICENSE_KEY            |                            | On-Prem licencni ključ.                                                                                                                            | Ne       |                                                             |
| GIPHY_API_KEY                  |                            | Giphy API ključ. Ako nije naveden, trebate kreirati konfiguraciono pravilo koje onemogućava izbor gifova.                                          | Ne       |                                                             |
| GIPHY_DEFAULT_RATING           | pg                         | Koristi se za Giphy integraciju. Može se takođe prebrisati pravilima za prilagođavanje widgeta.                                                    | Ne       | g, pg, pg-13, r                                             |
| OPENAI_SECRET_KEY              |                            | Koristi se za OpenAI funkcionalnosti poput opcionog GPT-baziranog detektovanja spama.                                                              | Ne       |                                                             |
| CDN_HOST_ADDR                  |                            | Hostname sa kojeg će se preuzimati asseti. Po defaultu koristi vrijednost HOSTNAME.                                                               | Ne       | example.com                                                 |
| LARGE_FILE_HOST_ADDR           |                            | Hostname sa kojeg se preuzimaju veliki fajlovi (npr. eksporti). Po defaultu koristi vrijednost CDN_HOST_ADDR.                                      | Ne       | example.com                                                 |
| LARGE_FILE_LOCATION_TYPE       | local_disk                 | Gdje treba čuvati velike fajlove, poput eksportovanih podataka.                                                                                    | Ne       | local_disk, s3                                              |
| FROM_EMAIL_HOST                |                            | Hostname sa kojeg bi emailovi trebali biti poslati.                                                                                               | Ne       | example.com                                                 |
| COOKIE_ID                      | fastcomments.sid           | Ime fastcomments kolačića.                                                                                                                         | Ne       |                                                             |
| COOKIE_HOSTNAME                | .fastcomments.com          | Vrijednost polja "hostname" u kolačiću. Preporučeno je prefiksirati tačkom.                                                                       | Ne       | .example.com                                                |
| S3_ACCESS_KEY                  |                            | Koristi se za korisničke uploadove fajlova, avatara itd. Po defaultu koristi lokalni FS ako nije definisano.                                      | Ne       |                                                             |
| S3_SECRET_KEY                  |                            | Koristi se za korisničke uploadove fajlova, avatara itd.                                                                                          | Ne       |                                                             |
| S3_REGION                      |                            | Koristi se za korisničke uploadove fajlova, avatara itd.                                                                                          | Ne       |                                                             |
| S3_BUCKET                      |                            | Koristi se za korisničke uploadove fajlova, avatara itd.                                                                                          | Ne       |                                                             |
| S3_HOST                        |                            | Koristi se za korisničke uploadove fajlova, avatara itd.                                                                                          | Ne       |                                                             |
| CACHE_DIR                      |                            | Lokacija za pohranu opcionalnog offline keša, za slučaj da DB nije dostupan. Periodično se osvježava sa top 100 nitima komentara.                   | Ne       |                                                             |
| BACKUP_DIR                     |                            | Lokacija za pohranu podataka za slučaj da DB nije dostupan. Ako je komentar poslan kada DB nije dostupan, ide ovdje i obrađuje se kasnije.         | Ne       |                                                             |

Imajte na umu da sve varijable vezane za domenu koriste sufiks `_HOST` ili `_ADDR`. Razlika je:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` treba da sadrži putanju do JSON fajla sa sljedećim primjerom formata:

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

U gore navedenom primjeru definišemo podrazumijevani `SMTP` email transport nazvan `mailgun`. Takođe definišemo specijalan transport koji koristimo posebno za `@yahoo.com` emailove. U nekim scenarijima poželjno je koristiti određenog provajdera ili IP adresu za slanje za određenu domenu kako bi se prilagodila isporuka. Ovo je opciono.

### DocumentDB

Prilikom povezivanja na `DocumentDB` preporučuje se postaviti `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` da biste bili kompatibilni sa podrazumijevanim postavkama.

---