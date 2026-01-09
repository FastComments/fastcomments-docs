FastComments koristi promenljive okruženja za konfiguraciju. Sledeća lista prikazuje sve podržane promenljive koje su relevantne za On-Prem.


| Promenljiva                    | Podrazumevano               | Informacije                                                                                                                                        | Obavezno | Primeri ili važeće vrednosti                         |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Tip okruženja.                                                                                                                                     | Da       | production, dev                                       |
| MONGO_URI                      |                             | URI za povezivanje sa bazom podataka.                                                                                                             | Da       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Omogućava korišćenje SSL-a za povezivanje sa bazom podataka.                                                                                       | Ne       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Omogućava validaciju sertifikata protiv CA prilikom povezivanja na Mongo.                                                                         | Ne       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem fajl.                                                                                                                             | Ne       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-pošta na koju treba da stignu važna sistemska obaveštenja.                                                                                      | Ne       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt za heširanje IP adresa.                                                                                                                       | Da       |                                                       |
| SESSION_SECRET                 |                             | Ključ koji se koristi za potpisivanje sesija.                                                                                                     | Da       |                                                       |
| SESSION_STORE_SECRET           |                             | Ključ koji se koristi za potpisivanje/heširanje sesija u skladištu. Mora biti različit od SESSION_SECRET.                                          | Da       |                                                       |
| HOSTNAME                       |                             | Ime hosta na kojem je FastComments postavljen (admin kontrolna tabla itd). NE bi trebalo da uključuje port ili protokol.                            | Da       | example.com                                           |
| HOST_ADDR                      |                             | Pristupačan URI na kojem je FastComments postavljen (admin kontrolna tabla itd).                                                                   | Da       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Putanja u lokalnom fajl sistemu do konfiguracije e-pošte (SMTP, mapiranja domena/provajdera itd).                                                  | Da       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Zaglavlje "From Name" u e-poruci.                                                                                                                  | Ne       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo u podnožju e-poruke.                                                                                                                          | Ne       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Zamena za "defaultTransport" u EMAIL_CONFIG_PATH. Korisno za postavljanje istog fajla konfiguracije u različitim okruženjima.                       | Ne       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID vašeg naloga na fastcomments.com. Koristi se za registraciju vašeg licencnog ključa.                                                           | Ne       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | On-prem licencni ključ.                                                                                                                            | Ne       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API ključ. Ako nije naveden, trebalo bi da napravite pravilo u konfiguraciji koje onemogućava izbor gif-ova.                                   | Ne       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Koristi se za Giphy integraciju. Može biti prepisano pravilima za prilagođavanje vidžeta.                                                         | Ne       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Koristi se za OpenAI funkcije kao što je opcionalna GPT zasnovana detekcija spama.                                                                 | Ne       |                                                       |
| CDN_HOST_ADDR                  |                             | Ime hosta odakle će se preuzimati resursi. Podrazumevano je vrednost HOSTNAME.                                                                   | Ne       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Ime hosta odakle će se preuzimati veliki fajlovi (poput eksportovanih podataka). Podrazumevano je vrednost CDN_HOST_ADDR.                           | Ne       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Gde treba čuvati velike fajlove, poput eksportovanih podataka.                                                                                     | Ne       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Ime hosta odakle treba slati e-poruke.                                                                                                             | Ne       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Ime fastcomments kolačića.                                                                                                                         | Ne       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Vrednost polja "hostname" kolačića. Preporučeno je prefiksirati tačkom.                                                                           | Ne       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Koristi se za korisnička otpremanja fajlova, avatare itd. Podrazumevano lokalni fajl sistem ako nije definisano.                                   | Ne       |                                                       |
| S3_SECRET_KEY                  |                             | Koristi se za korisnička otpremanja fajlova, avatare itd.                                                                                         | Ne       |                                                       |
| S3_REGION                      |                             | Koristi se za korisnička otpremanja fajlova, avatare itd.                                                                                         | Ne       |                                                       |
| S3_BUCKET                      |                             | Koristi se za korisnička otpremanja fajlova, avatare itd.                                                                                         | Ne       |                                                       |
| S3_HOST                        |                             | Koristi se za korisnička otpremanja fajlova, avatare itd.                                                                                         | Ne       |                                                       |
| CACHE_DIR                      |                             | Lokacija za opcioni offline keš, za slučaj da DB nije dostupan. Periodično se osvežava sa top 100 niti komentara.                                   | Ne       |                                                       |
| BACKUP_DIR                     |                             | Lokacija za čuvanje podataka za slučaj da DB nije dostupan. Ako je komentar poslat dok DB nije dostupan, ide ovde i obrađuje se kasnije.           | Ne       |                                                       |

Napomena da sve promenljive vezane za domene koriste sufikse `_HOST` ili `_ADDR`. Razlika je:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` treba da sadrži putanju do JSON fajla sa sledećim primerom formata:

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

U gornjem primeru definišemo podrazumevani `SMTP` email transport nazvan `mailgun`. Takođe definišemo specijalan transport koji koristimo posebno za `@yahoo.com` adrese. U nekim scenarijima poželjno je koristiti specifičnog provajdera ili IP za slanje za određeni domen kako bi se podesila isporuka. Ovo je opciono.

### DocumentDB

Kada se povezujete na `DocumentDB` treba da navedete `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` da biste bili kompatibilni sa podrazumevanim podešavanjima.

---