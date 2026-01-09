FastComments utilizza variabili d'ambiente per la configurazione. La seguente lista illustra tutte le variabili supportate rilevanti per On-Prem.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Tipo di ambiente.                                                                                                                                  | Sì       | production, dev                                       |
| MONGO_URI                      |                             | URI di connessione al DB.                                                                                                                          | Sì       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Abilita l'uso di SSL per connettersi al database.                                                                                                 | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Abilita la validazione del certificato rispetto alla CA quando ci si connette a Mongo.                                                            | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | File pem CA per SSL di Mongo.                                                                                                                      | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email dove inviare notifiche importanti relative al sistema.                                                                                      | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt per l'hashing degli indirizzi IP.                                                                                                            | Sì       |                                                       |
| SESSION_SECRET                 |                             | Chiave usata per firmare le sessioni.                                                                                                              | Sì       |                                                       |
| SESSION_STORE_SECRET           |                             | Chiave usata per firmare/hashare le sessioni nello storage. Deve essere diversa da SESSION_SECRET.                                                | Sì       |                                                       |
| HOSTNAME                       |                             | Il nome host dove è distribuito FastComments (dashboard admin ecc.). NON deve includere porta o protocollo.                                       | Sì       | example.com                                           |
| HOST_ADDR                      |                             | Un URI accessibile dove è distribuito FastComments (dashboard admin ecc.).                                                                        | Sì       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Un percorso nel file system locale dove si trova la configurazione email (SMTP, mapping dominio/fornitore, ecc.).                                   | Sì       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Header "From Name" delle email.                                                                                                                    | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo nel footer delle email.                                                                                                                       | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Override per "defaultTransport" in EMAIL_CONFIG_PATH. Utile per distribuire lo stesso file di configurazione in ambienti diversi.                   | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | L'ID del tuo account su fastcomments.com. Usato per registrare la tua license key.                                                                | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Una license key per on-prem.                                                                                                                       | No       |                                                       |
| GIPHY_API_KEY                  |                             | Chiave API Giphy. Se non specificata, dovresti creare una regola di configurazione che disabiliti il selettore di gif.                             | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Usato per l'integrazione Giphy. Può anche essere sovrascritto con regole di personalizzazione del widget.                                         | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Usato per funzionalità basate su OpenAI come il rilevamento spam opzionale basato su GPT.                                                         | No       |                                                       |
| CDN_HOST_ADDR                  |                             | Il nome host da cui verranno recuperate le risorse. Di default prende il valore di HOSTNAME.                                                     | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Il nome host da cui verranno recuperati i file di grandi dimensioni (come gli export). Di default prende il valore di CDN_HOST_ADDR.               | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Dove dovrebbero essere memorizzati i file di grandi dimensioni, come gli export.                                                                  | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Il nome host da cui dovrebbero essere inviate le email.                                                                                           | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Nome del cookie fastcomments.                                                                                                                      | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Valore del campo "hostname" del cookie. Si raccomanda di anteporre un punto.                                                                      | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Usato per upload file degli utenti, avatar, ecc. Di default usa il filesystem locale se non definito.                                             | No       |                                                       |
| S3_SECRET_KEY                  |                             | Usato per upload file degli utenti, avatar, ecc.                                                                                                  | No       |                                                       |
| S3_REGION                      |                             | Usato per upload file degli utenti, avatar, ecc.                                                                                                  | No       |                                                       |
| S3_BUCKET                      |                             | Usato per upload file degli utenti, avatar, ecc.                                                                                                  | No       |                                                       |
| S3_HOST                        |                             | Usato per upload file degli utenti, avatar, ecc.                                                                                                  | No       |                                                       |
| CACHE_DIR                      |                             | Posizione per memorizzare la cache offline opzionale, per quando il DB non è disponibile. Viene aggiornato periodicamente con le top 100 discussioni.| No       |                                                       |
| BACKUP_DIR                     |                             | Posizione per memorizzare i dati per quando il DB non è disponibile. Se un commento viene inviato quando il DB non è disponibile, finisce qui e viene processato dopo. | No       |                                                       |

Nota che tutte le variabili relative al dominio utilizzano il suffisso `_HOST` o `_ADDR`. La differenza è:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Il `EMAIL_CONFIG_PATH` dovrebbe contenere un percorso a un file JSON con il seguente formato di esempio:

[inline-code-attrs-start title = 'Configurazione Email'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Nell'esempio sopra definiamo un trasporto email SMTP di default chiamato `mailgun`. Definiamo anche un trasporto speciale che usiamo specificamente per le email `@yahoo.com`. In alcuni scenari è desiderabile usare un provider o un indirizzo IP di invio specifico per un dominio per ottimizzare la consegna. Questo è opzionale.

### DocumentDB

Quando ti connetti a `DocumentDB` vorrai specificare `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` per essere compatibile con le impostazioni di default.

---