FastComments verwendet Umgebungsvariablen zur Konfiguration. Die folgende Liste beschreibt alle unterstützten Variablen, die für On-Prem relevant sind.


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Umgebungstyp.                                                                                                                                      | Ja       | production, dev                                       |
| MONGO_URI                      |                             | DB-Verbindungs-URI.                                                                                                                                | Ja       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Aktiviert die Verwendung von SSL zur Verbindung mit der Datenbank.                                                                                 | Nein     | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Aktiviert die Validierung des Zertifikats gegenüber der CA bei der Verbindung zu Mongo.                                                            | Nein     | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem Datei.                                                                                                                            | Nein     | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-Mail, an die wichtige systembezogene Benachrichtigungen gesendet werden sollen.                                                                 | Nein     | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt zum Hashen von IP-Adressen.                                                                                                                  | Ja       |                                                       |
| SESSION_SECRET                 |                             | Der Schlüssel, mit dem Sessions signiert werden.                                                                                                  | Ja       |                                                       |
| SESSION_STORE_SECRET           |                             | Der Schlüssel, mit dem Sessions im Speicher signiert/gehasht werden. Muss sich von SESSION_SECRET unterscheiden.                                   | Ja       |                                                       |
| HOSTNAME                       |                             | Der Hostname, unter dem FastComments bereitgestellt wird (Admin-Dashboard usw.). Sollte KEINE Port- oder Protokollangabe enthalten.                   | Ja       | example.com                                           |
| HOST_ADDR                      |                             | Eine erreichbare URI, unter der FastComments bereitgestellt wird (Admin-Dashboard usw.).                                                           | Ja       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Ein Pfad im lokalen Dateisystem, in dem die E-Mail-Konfiguration (SMTP, Domain/Provider-Zuordnungen usw.) liegt.                                   | Ja       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | E-Mail Header "From Name".                                                                                                                         | Nein     | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo in der E-Mail-Fußzeile.                                                                                                                       | Nein     | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Überschreibung für "defaultTransport" in EMAIL_CONFIG_PATH. Nützlich, um dieselbe Konfigurationsdatei in verschiedenen Umgebungen bereitzustellen. | Nein     | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | Die ID Ihres Kontos auf fastcomments.com. Wird zur Registrierung Ihres Lizenzschlüssels verwendet.                                                | Nein     |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Ein On-Prem-Lizenzschlüssel.                                                                                                                       | Nein     |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API-Schlüssel. Falls nicht angegeben, sollten Sie eine Konfigurationsregel erstellen, die den GIF-Picker deaktiviert.                        | Nein     |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Wird für die Giphy-Integration verwendet. Kann auch mit Widget-Anpassungsregeln überschrieben werden.                                             | Nein     | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Wird für OpenAI-basierte Funktionen wie optionalen GPT-gestützten Spam-Schutz verwendet.                                                          | Nein     |                                                       |
| CDN_HOST_ADDR                  |                             | Der Hostname, von dem Assets abgerufen werden. Standardmäßig der Wert von HOSTNAME.                                                               | Nein     | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Der Hostname, von dem große Dateien (wie Exporte) abgerufen werden. Standardmäßig der Wert von CDN_HOST_ADDR.                                      | Nein     | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Wo große Dateien, wie Exporte, gespeichert werden sollen.                                                                                         | Nein     | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Der Hostname, von dem E-Mails versendet werden sollen.                                                                                             | Nein     | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Der Name des fastcomments Cookies.                                                                                                                 | Nein     |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Der Wert des Cookie-Feldes "hostname". Es wird empfohlen, einen Punkt vorzuschalten.                                                              | Nein     | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Wird für Benutzerdatei-Uploads, Avatare usw. verwendet. Standardmäßig lokales Dateisystem, falls undefiniert.                                      | Nein     |                                                       |
| S3_SECRET_KEY                  |                             | Wird für Benutzerdatei-Uploads, Avatare usw. verwendet.                                                                                            | Nein     |                                                       |
| S3_REGION                      |                             | Wird für Benutzerdatei-Uploads, Avatare usw. verwendet.                                                                                            | Nein     |                                                       |
| S3_BUCKET                      |                             | Wird für Benutzerdatei-Uploads, Avatare usw. verwendet.                                                                                            | Nein     |                                                       |
| S3_HOST                        |                             | Wird für Benutzerdatei-Uploads, Avatare usw. verwendet.                                                                                            | Nein     |                                                       |
| CACHE_DIR                      |                             | Speicherort für optionalen Offline-Cache, wenn die DB nicht verfügbar ist. Wird periodisch mit den 100 wichtigsten Kommentar-Threads aktualisiert. | Nein     |                                                       |
| BACKUP_DIR                     |                             | Speicherort für Daten, wenn die DB nicht verfügbar ist. Wenn ein Kommentar eingereicht wird, während die DB nicht verfügbar ist, landet er hier und wird später verarbeitet. | Nein     |                                                       |

Beachten Sie, dass alle domänenbezogenen Variablen das Postfix `_HOST` oder `_ADDR` verwenden. Der Unterschied ist:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Der `EMAIL_CONFIG_PATH` sollte einen Pfad zu einer JSON-Datei mit dem folgenden Beispiel-Format enthalten:

[inline-code-attrs-start title = 'E-Mail-Konfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Im obigen Beispiel definieren wir einen Standard-`SMTP`-E-Mail-Transport namens `mailgun`. Außerdem definieren wir einen speziellen Transport, den wir speziell für `@yahoo.com`-E-Mails verwenden. In einigen Szenarien ist es wünschenswert, für eine Domain einen bestimmten Provider oder eine bestimmte Versand-IP zu verwenden, um die Zustellung zu optimieren. Dies ist optional.

### DocumentDB

Beim Verbinden mit `DocumentDB` sollten Sie `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` angeben, um mit den Standardeinstellungen kompatibel zu sein.

---