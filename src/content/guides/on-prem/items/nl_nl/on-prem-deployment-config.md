FastComments gebruikt omgevingsvariabelen voor configuratie. De volgende lijst geeft alle ondersteunde variabelen weer die relevant zijn voor On-Prem.


| Variabele                      | Standaard                   | Info                                                                                                                                               | Verplicht | Voorbeelden of Geldige Waarden                          |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Omgevingstype.                                                                                                                                     | Ja       | production, dev                                       |
| MONGO_URI                      |                             | DB-verbinding URI.                                                                                                                                | Ja       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Schakelt het gebruik van SSL in om verbinding te maken met de database.                                                                          | Nee      | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Schakelt validatie van het certificaat tegen de CA in bij het verbinden met Mongo.                                                               | Nee      | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA PEM-bestand.                                                                                                                         | Nee      | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-mail waar belangrijke systeemgerelateerde meldingen naartoe moeten worden gestuurd.                                                            | Nee      | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt voor het hashen van IP-adressen.                                                                                                            | Ja       |                                                       |
| SESSION_SECRET                 |                             | De sleutel die wordt gebruikt om sessies te ondertekenen.                                                                                        | Ja       |                                                       |
| SESSION_STORE_SECRET           |                             | De sleutel die wordt gebruikt om sessies in opslag te ondertekenen/hashten. Moet anders zijn dan SESSION_SECRET.                                  | Ja       |                                                       |
| HOSTNAME                       |                             | De hostnaam waar FastComments is geïmplementeerd (admin-dashboard enz.). Mag NIET poort of protocol bevatten.                                     | Ja       | example.com                                           |
| HOST_ADDR                      |                             | Een toegankelijke URI waar FastComments is ingezet (admin-dashboard enz.).                                                                      | Ja       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Een pad op het lokale bestandssysteem waar de e-mailconfiguratie (SMTP, domein/provider-koppelingen, enz.) staat.                                 | Ja       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | E-mail "From Name"-header.                                                                                                                        | Nee      | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | E-mail voettekstlogo.                                                                                                                             | Nee      | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Overschrijving voor "defaultTransport" in EMAIL_CONFIG_PATH. Handig om hetzelfde configuratiebestand in verschillende omgevingen te gebruiken.     | Nee      | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | De ID van uw account op fastcomments.com. Gebruikt om uw licentiesleutel te registreren.                                                         | Nee      |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Een on-prem licentiesleutel.                                                                                                                      | Nee      |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API-sleutel. Als deze niet is opgegeven, moet u een configuratieregel maken die de gifkiezer uitschakelt.                                   | Nee      |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Gebruikt voor giphy-integratie. Kan ook worden overschreven met widget-aanpassingsregels.                                                         | Nee      | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Gebruikt voor OpenAI-aangedreven functies zoals optionele GPT-gebaseerde spamdetectie.                                                           | Nee      |                                                       |
| CDN_HOST_ADDR                  |                             | De hostnaam van waar assets worden opgehaald. Standaard waarde is HOSTNAME.                                                                      | Nee      | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | De hostnaam waar grote bestanden (zoals exports) worden opgehaald. Standaard waarde is CDN_HOST_ADDR.                                             | Nee      | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Waar grote bestanden, zoals exports, moeten worden opgeslagen.                                                                                    | Nee      | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | De hostnaam waar e-mails van verzonden moeten worden.                                                                                             | Nee      | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | De naam van de fastcomments-cookie.                                                                                                               | Nee      |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | De waarde van het "hostname"-veld van de cookie. Aanbevolen te prefixen met een punt.                                                            | Nee      | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Wordt gebruikt voor gebruikersbestandsuploads, avatars, enz. Standaard lokale FS als niet gedefinieerd.                                            | Nee      |                                                       |
| S3_SECRET_KEY                  |                             | Wordt gebruikt voor gebruikersbestandsuploads, avatars, enz.                                                                                      | Nee      |                                                       |
| S3_REGION                      |                             | Wordt gebruikt voor gebruikersbestandsuploads, avatars, enz.                                                                                      | Nee      |                                                       |
| S3_BUCKET                      |                             | Wordt gebruikt voor gebruikersbestandsuploads, avatars, enz.                                                                                      | Nee      |                                                       |
| S3_HOST                        |                             | Wordt gebruikt voor gebruikersbestandsuploads, avatars, enz.                                                                                      | Nee      |                                                       |
| CACHE_DIR                      |                             | Locatie om optionele offline cache op te slaan, voor wanneer de DB niet beschikbaar is. Periodiek ververst met de top 100 reactiedraden.           | Nee      |                                                       |
| BACKUP_DIR                     |                             | Locatie om gegevens op te slaan voor wanneer de DB niet beschikbaar is. Als een reactie wordt ingediend wanneer de DB niet beschikbaar is, gaat deze hierheen en wordt later verwerkt. | Nee      |                                                       |

Let op dat alle domeingerelateerde variabelen de achtervoegsels `_HOST` of `_ADDR` gebruiken. Het verschil is:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Het `EMAIL_CONFIG_PATH` moet een pad naar een JSON-bestand bevatten met het volgende voorbeeldformaat:

[inline-code-attrs-start title = 'E-mailconfiguratie'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

In het bovenstaande voorbeeld definiëren we een standaard `SMTP` e-mailtransport genaamd `mailgun`. We definiëren ook een speciaal transport dat we specifiek gebruiken voor `@yahoo.com`-e-mails. In sommige scenario's is het wenselijk om voor een domein een specifieke provider of verzend-IP te gebruiken om de aflevering te optimaliseren. Dit is optioneel.

### DocumentDB

Bij het verbinden met `DocumentDB` wilt u `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` opgeven om compatibel te zijn met de standaardinstellingen.

---