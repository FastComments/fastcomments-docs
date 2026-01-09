FastComments bruger miljøvariabler til konfiguration. Følgende liste skitserer alle understøttede variabler, der er relevante for On-Prem.


| Variabel                       | Standard                    | Info                                                                                                                                               | Påkrævet | Eksempler eller gyldige værdier                         |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Miljøtype.                                                                                                                                         | Ja       | production, dev                                       |
| MONGO_URI                      |                             | DB-forbindelses-URI.                                                                                                                               | Ja       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Aktiverer brug af SSL til at oprette forbindelse til databasen.                                                                                    | Nej      | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Aktiverer validering af certifikatet mod CA'en ved forbindelse til Mongo.                                                                          | Nej      | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem-fil.                                                                                                                              | Nej      | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-mailadresse hvor vigtige systemrelaterede notifikationer sendes.                                                                                 | Nej      | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt til hashing af IP-adresser.                                                                                                                   | Ja       |                                                       |
| SESSION_SECRET                 |                             | Nøglen, der bruges til at signere sessioner.                                                                                                       | Ja       |                                                       |
| SESSION_STORE_SECRET           |                             | Nøglen, der bruges til at signere/hash'e sessioner i storage. Skal være forskellig fra SESSION_SECRET.                                             | Ja       |                                                       |
| HOSTNAME                       |                             | Værtsnavnet hvor FastComments er deployeret (admin dashboard osv.). Skal IKKE inkludere port eller protokol.                                       | Ja       | example.com                                           |
| HOST_ADDR                      |                             | En tilgængelig URI hvor FastComments er deployeret (admin dashboard osv.).                                                                         | Ja       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | En sti på det lokale filsystem hvor e-mailkonfigurationen (SMTP, domæne/udbyder-tilknytninger mv.) ligger.                                          | Ja       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | E-mailens "From Name"-header.                                                                                                                      | Nej      | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo i e-mailens sidefod.                                                                                                                          | Nej      | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Override for "defaultTransport" i EMAIL_CONFIG_PATH. Nyttigt til at deploye samme konfigurationsfil til forskellige miljøer.                       | Nej      | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID'et for din konto på fastcomments.com. Bruges til at registrere din licensnøgle.                                                                 | Nej      |                                                       |
| ON_PREM_LICENSE_KEY            |                             | En on-prem licensnøgle.                                                                                                                             | Nej      |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API-nøgle. Hvis ikke angivet, bør du oprette en konfigurationsregel som deaktiverer GIF-vælgeren.                                           | Nej      |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Bruges til giphy-integration. Kan også overskrives med widget-tilpasningsregler.                                                                  | Nej      | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Bruges til openai-drevne funktioner som valgfri GPT-baseret spamdetektion.                                                                        | Nej      |                                                       |
| CDN_HOST_ADDR                  |                             | Værtsnavnet hvor assets hentes fra. Standard er værdien af HOSTNAME.                                                                              | Nej      | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Værtsnavnet hvor store filer (som eksporter) hentes fra. Standard er værdien af CDN_HOST_ADDR.                                                     | Nej      | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Hvor store filer, såsom eksporter, skal gemmes.                                                                                                    | Nej      | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Værtsnavnet, som e-mails skal sendes fra.                                                                                                          | Nej      | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Navnet på fastcomments-cookien.                                                                                                                     | Nej      |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Værdien af cookieens "hostname"-felt. Anbefales at præfikse med punktum.                                                                          | Nej      | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Bruges til brugeres filuploads, avatarer osv. Standard er lokalt filsystem hvis udefineret.                                                        | Nej      |                                                       |
| S3_SECRET_KEY                  |                             | Bruges til brugeres filuploads, avatarer osv.                                                                                                      | Nej      |                                                       |
| S3_REGION                      |                             | Bruges til brugeres filuploads, avatarer osv.                                                                                                      | Nej      |                                                       |
| S3_BUCKET                      |                             | Bruges til brugeres filuploads, avatarer osv.                                                                                                      | Nej      |                                                       |
| S3_HOST                        |                             | Bruges til brugeres filuploads, avatarer osv.                                                                                                      | Nej      |                                                       |
| CACHE_DIR                      |                             | Placering til at gemme en valgfri offline-cache, til når DB ikke er tilgængelig. Periodisk opdateret med top 100 kommentartråde.                   | Nej      |                                                       |
| BACKUP_DIR                     |                             | Placering til at gemme data når DB ikke er tilgængelig. Hvis en kommentar indsendes når DB ikke er tilgængelig, går den herhen og behandles senere. | Nej      |                                                       |

Bemærk at alle domænerelaterede variabler bruger suffikset `_HOST` eller `_ADDR`. Forskellen er:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

The `EMAIL_CONFIG_PATH` should contain a path to a JSON file with the following example format:

[inline-code-attrs-start title = 'E-mail-konfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

I ovenstående eksempel definerer vi en standard `SMTP` e-mailtransport kaldet `mailgun`. Vi definerer også en specialtransport, som vi bruger specifikt til `@yahoo.com`-e-mails. I nogle scenarier kan det være ønskeligt at bruge en specifik udbyder eller afsender-IP for et domæne for at finjustere levering. Dette er valgfrit.

### DocumentDB

Når du opretter forbindelse til `DocumentDB` bør du angive `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` for at være kompatibel med standardindstillingerne.

---