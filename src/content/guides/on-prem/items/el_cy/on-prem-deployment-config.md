Το FastComments χρησιμοποιεί μεταβλητές περιβάλλοντος για διαμόρφωση. Η παρακάτω λίστα περιγράφει όλες τις υποστηριζόμενες μεταβλητές που σχετίζονται με την On-Prem.


| Μεταβλητή                     | Προεπιλογή                 | Πληροφορίες                                                                                                                                         | Απαραίτητο | Παραδείγματα ή Έγκυρες Τιμές                          |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Τύπος περιβάλλοντος.                                                                                                                               | Ναι      | production, dev                                       |
| MONGO_URI                      |                             | URI σύνδεσης βάσης δεδομένων.                                                                                                                      | Ναι      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Ενεργοποιεί τη χρήση SSL για σύνδεση στη βάση δεδομένων.                                                                                            | Όχι      | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Ενεργοποιεί την επαλήθευση του πιστοποιητικού έναντι της CA κατά τη σύνδεση στο Mongo.                                                              | Όχι      | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA pem αρχείο.                                                                                                                           | Όχι      | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email στο οποίο θα αποστέλλονται σημαντικές ειδοποιήσεις σχετικές με το σύστημα.                                                                  | Όχι      | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt για hashing διευθύνσεων IP.                                                                                                                   | Ναι      |                                                       |
| SESSION_SECRET                 |                             | Το κλειδί που χρησιμοποιείται για την υπογραφή των συνεδριών.                                                                                       | Ναι      |                                                       |
| SESSION_STORE_SECRET           |                             | Το κλειδί που χρησιμοποιείται για την υπογραφή/κατακερματισμό των συνεδριών στο storage. Πρέπει να είναι διαφορετικό από SESSION_SECRET.                 | Ναι      |                                                       |
| HOSTNAME                       |                             | Το hostname όπου έχει αναπτυχθεί το FastComments (π.χ. admin dashboard). Δεν πρέπει να περιλαμβάνει θύρα ή πρωτόκολλο.                                | Ναι      | example.com                                           |
| HOST_ADDR                      |                             | Μια προσβάσιμη URI όπου έχει αναπτυχθεί το FastComments (π.χ. admin dashboard).                                                                    | Ναι      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Μια διαδρομή στο τοπικό σύστημα αρχείων όπου βρίσκεται η διαμόρφωση email (SMTP, αντιστοιχίσεις domain/provider, κ.λπ.).                              | Ναι      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Email "From Name" header.                                                                                                                          | Όχι      | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Email footer logo.                                                                                                                                 | Όχι      | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Override για το "defaultTransport" στο EMAIL_CONFIG_PATH. Χρήσιμο για ανάπτυξη του ίδιου αρχείου ρύθμισης σε διαφορετικά περιβάλλοντα.                 | Όχι      | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | Το ID του λογαριασμού σας στο fastcomments.com. Χρησιμοποιείται για την εγγραφή του license key.                                                   | Όχι      |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Ένα on-prem license key.                                                                                                                            | Όχι      |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Key. Αν δεν καθοριστεί, πρέπει να δημιουργήσετε κανόνα ρύθμισης που να απενεργοποιεί τον επιλογέα gif.                                      | Όχι      |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Χρησιμοποιείται για ενσωμάτωση με το Giphy. Μπορεί επίσης να παρακαμφθεί με κανόνες εξατομίκευσης του widget.                                         | Όχι      | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Χρησιμοποιείται για λειτουργίες με OpenAI, όπως προαιρετικός εντοπισμός spam βασισμένος σε GPT.                                                       | Όχι      |                                                       |
| CDN_HOST_ADDR                  |                             | Το hostname από όπου θα ανακτώνται τα assets. Προεπιλογή στην τιμή του HOSTNAME.                                                                    | Όχι      | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Το hostname από όπου ανακτώνται μεγάλα αρχεία (π.χ. εξαγωγές). Προεπιλογή στην τιμή του CDN_HOST_ADDR.                                              | Όχι      | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Πού θα αποθηκεύονται μεγάλα αρχεία, όπως εξαγωγές.                                                                                                  | Όχι      | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Το hostname από το οποίο θα αποστέλλονται τα emails.                                                                                                | Όχι      | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Το όνομα του cookie του fastcomments.                                                                                                               | Όχι      |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Η τιμή του πεδίου "hostname" του cookie. Συνιστάται να προστεθεί προθέμα τελείας.                                                                    | Όχι      | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Χρησιμοποιείται για ανέβασμα αρχείων από χρήστες, avatars, κ.λπ. Προεπιλογή στο τοπικό FS αν δεν οριστεί.                                             | Όχι      |                                                       |
| S3_SECRET_KEY                  |                             | Χρησιμοποιείται για ανέβασμα αρχείων από χρήστες, avatars, κ.λπ.                                                                                    | Όχι      |                                                       |
| S3_REGION                      |                             | Χρησιμοποιείται για ανέβασμα αρχείων από χρήστες, avatars, κ.λπ.                                                                                    | Όχι      |                                                       |
| S3_BUCKET                      |                             | Χρησιμοποιείται για ανέβασμα αρχείων από χρήστες, avatars, κ.λπ.                                                                                    | Όχι      |                                                       |
| S3_HOST                        |                             | Χρησιμοποιείται για ανέβασμα αρχείων από χρήστες, avatars, κ.λπ.                                                                                    | Όχι      |                                                       |
| CACHE_DIR                      |                             | Τοποθεσία για αποθήκευση προαιρετικού offline cache, για όταν η DB δεν είναι διαθέσιμη. Ενημερώνεται περιοδικά με τα 100 κορυφαία νήματα σχολίων.      | Όχι      |                                                       |
| BACKUP_DIR                     |                             | Τοποθεσία για αποθήκευση δεδομένων όταν η DB δεν είναι διαθέσιμη. Αν ένα σχόλιο υποβληθεί όταν η DB δεν είναι διαθέσιμη, αποθηκεύεται εδώ και επεξεργάζεται αργότερα. | Όχι      |                                                       |

Σημειώστε ότι όλες οι μεταβλητές που σχετίζονται με domain χρησιμοποιούν την κατάληξη `_HOST` ή `_ADDR`. Η διαφορά είναι:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Το `EMAIL_CONFIG_PATH` πρέπει να περιέχει μια διαδρομή προς ένα αρχείο JSON με το ακόλουθο παράδειγμα μορφής:

[inline-code-attrs-start title = 'Ρυθμίσεις Email'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Στο παραπάνω παράδειγμα ορίζουμε έναν προεπιλεγμένο μεταφορέα email `SMTP` με όνομα `mailgun`. Επίσης ορίζουμε έναν ειδικό μεταφορέα που χρησιμοποιείται ειδικά για emails `@yahoo.com`. Σε ορισμένα σενάρια είναι επιθυμητό να χρησιμοποιηθεί συγκεκριμένος πάροχος ή διεύθυνση IP αποστολής για ένα domain προκειμένου να βελτιωθεί η παράδοση. Αυτό είναι προαιρετικό.

### DocumentDB

Όταν συνδέεστε στο `DocumentDB` θα θέλετε να ορίσετε `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` για να είστε συμβατοί με τις προεπιλεγμένες ρυθμίσεις.

---