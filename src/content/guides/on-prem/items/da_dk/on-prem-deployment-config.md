FastComments bruger miljøvariabler til konfiguration. Følgende liste beskriver alle understøttede variabler, der er relevante for On-Prem.


| Variabel                       | Standard                    | Info                                                                                                                                               | Påkrævet | Eksempler eller gyldige værdier                       |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Miljøtype.                                                                                                                                  | Ja      | production, dev                                       |
| MONGO_URI                      |                             | DB-forbindelses-URI.                                                                                                                                 | Ja      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Aktiverer brug af SSL til at forbinde til databasen.                                                                                                      | Nej       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Aktiverer validering af certifikatet mod CA'en ved forbindelse til Mongo.                                                                               | Nej       | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem-fil.                                                                                                                             | Nej       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-mail, hvor vigtige systemrelaterede notifikationer skal sendes.                                                                                      | Nej       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt til hashing af IP-adresser.                                                                                                                     | Ja      |                                                       |
| SESSION_SECRET                 |                             | Nøglen, der bruges til at signere sessioner.                                                                                                                     | Ja      |                                                       |
| SESSION_STORE_SECRET           |                             | Nøglen, der bruges til at signere/hash sessioner i lageret. Skal være forskellig fra SESSION_SECRET.                                                              | Ja      |                                                       |
| HOSTNAME                       |                             | Værtsnavnet, hvor FastComments er implementeret (admin-dashboard osv.). Skal IKKE indeholde port eller protokol.                                            | Ja      | example.com                                           |
| HOST_ADDR                      |                             | En tilgængelig URI, hvor FastComments er implementeret (admin-dashboard osv.).                                                                            | Ja      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | En sti på det lokale filsystem, hvor e-mail-konfigurationen (SMTP, domæne/udbyder-mappings osv.) befinder sig.                                                   | Ja      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | E-mail-headeren "From Name".                                                                                                                          | Nej       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo i e-mailens sidefod.                                                                                                                                 | Nej       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Overskrivning af "defaultTransport" i EMAIL_CONFIG_PATH. Nyttig ved implementering af samme konfigurationsfil til forskellige miljøer.                                     | Nej       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ID'et for din konto på fastcomments.com. Bruges til at registrere din licensnøgle.                                                                     | Nej       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | En on-prem licensnøgle.                                                                                                                            | Nej       |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API-nøgle. Hvis den ikke er angivet, bør du oprette en konfigurationsregel, der deaktiverer gif-vælgeren.                                                     | Nej       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Bruges til giphy-integration. Kan også overskrives med widget-tilpasningsregler.                                                                | Nej       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Bruges til openai-drevne funktioner som valgfri GPT-baseret spamdetektion.                                                                           | Nej       |                                                       |
| CDN_HOST_ADDR                  |                             | Værtsnavnet, hvorfra assets hentes. Standard er værdien af HOSTNAME.                                                                     | Nej       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Værtsnavnet, hvorfra store filer (som eksporter) hentes. Standard er værdien af CDN_HOST_ADDR.                                                | Nej       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Hvor store filer, som eksporter, skal gemmes.                                                                                                 | Nej       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Værtsnavnet, hvorfra e-mails skal sendes.                                                                                                     | Nej       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Navnet på fastcomments-cookien.                                                                                                       | Nej       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Værdien af cookiens "hostname"-felt. Anbefales at prefikse med et punktum.                                                                          | Nej       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Bruges til brugerens filuploads, avatarer osv. Standard er lokalt filsystem, hvis undefined.                                                                       | Nej       |                                                       |
| S3_SECRET_KEY                  |                             | Bruges til brugerens filuploads, avatarer osv.                                                                                                          | Nej       |                                                       |
| S3_REGION                      |                             | Bruges til brugerens filuploads, avatarer osv.                                                                                                          | Nej       |                                                       |
| S3_BUCKET                      |                             | Bruges til brugerens filuploads, avatarer osv.                                                                                                          | Nej       |                                                       |
| S3_HOST                        |                             | Bruges til brugerens filuploads, avatarer osv.                                                                                                          | Nej       |                                                       |
| CACHE_DIR                      |                             | Placering til at gemme valgfri offline cache, til når databasen ikke er tilgængelig. Opdateres periodisk med de 100 mest populære kommentartråde.                       | Nej       |                                                       |
| BACKUP_DIR                     |                             | Placering til at gemme data til når databasen ikke er tilgængelig. Hvis en kommentar indsendes, når databasen ikke er tilgængelig, lander den her og behandles senere. | Nej       |                                                       |

Bemærk, at alle domænerelaterede variabler bruger endelsen `_HOST` eller `_ADDR`. Forskellen er:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

`EMAIL_CONFIG_PATH` skal indeholde en sti til en JSON-fil med følgende eksempelformat:

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

I ovenstående eksempel definerer vi en standard `SMTP` e-mail-transport kaldet `mailgun`. Vi definerer også en speciel transport, som vi bruger specifikt til
`@yahoo.com` e-mails. I visse scenarier er det ønskeligt at bruge en specifik udbyder eller afsender-IP for et domæne for at finjustere leveringen. Dette er valgfrit.

### DocumentDB

Når du forbinder til `DocumentDB`, skal du angive `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` for at være kompatibel med standardindstillingerne.