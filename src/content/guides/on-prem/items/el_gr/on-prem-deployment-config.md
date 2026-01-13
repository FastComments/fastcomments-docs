FastComments χρησιμοποιεί μεταβλητές περιβάλλοντος για τη διαμόρφωση. Η παρακάτω λίστα απαριθμεί όλες τις υποστηριζόμενες μεταβλητές που είναι σχετικές με On-Prem.


| Μεταβλητή                      | Προεπιλογή                  | Πληροφορίες                                                                                                                                       | Απαραίτητο | Παραδείγματα ή Έγκυρες Τιμές                          |
|--------------------------------|-----------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|------------|-------------------------------------------------------|
| NODE_ENV                       |                             | Τύπος περιβάλλοντος.                                                                                                                              | Ναι        | production, dev                                       |
| MONGO_URI                      |                             | DB Connection URI.                                                                                                                               | Ναι        |                                                       |
| MONGO_ENABLE_SSL               | false                       | Ενεργοποιεί τη χρήση SSL για σύνδεση με τη βάση δεδομένων.                                                                                        | Όχι        | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Ενεργοποιεί την επαλήθευση του πιστοποιητικού έναντι του CA κατά τη σύνδεση με το Mongo.                                                           | Όχι        | true, false                                           |
| MONGO_SSL_CA                   |                             | Mongo SSL CA αρχείο pem.                                                                                                                         | Όχι        | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Email όπου πρέπει να αποστέλλονται σημαντικές ειδοποιήσεις συστήματος.                                                                            | Όχι        | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt για το hashing διευθύνσεων IP.                                                                                                              | Ναι        |                                                       |
| SESSION_SECRET                 |                             | Το κλειδί που χρησιμοποιείται για την υπογραφή των sessions.                                                                                     | Ναι        |                                                       |
| SESSION_STORE_SECRET           |                             | Το κλειδί που χρησιμοποιείται για την υπογραφή/το hashing των sessions στην αποθήκευση. Πρέπει να είναι διαφορετικό από το SESSION_SECRET.           | Ναι        |                                                       |
| HOSTNAME                       |                             | Το hostname όπου έχει αναπτυχθεί το FastComments (π.χ. admin dashboard). Δεν πρέπει να περιλαμβάνει port ή πρωτόκολλο.                             | Ναι        | example.com                                           |
| HOST_ADDR                      |                             | Ένα προσβάσιμο URI όπου έχει αναπτυχθεί το FastComments (π.χ. admin dashboard).                                                                  | Ναι        | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Μια διαδρομή στο τοπικό σύστημα αρχείων όπου βρίσκεται η ρύθμιση email (SMTP, αντιστοιχίσεις domain/provider, κ.λπ.).                             | Ναι        | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Κεφαλίδα "From Name" στα email.                                                                                                                  | Όχι        | Το όνομα της εταιρείας μου                            |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Λογότυπο στο υποσέλιδο του email.                                                                                                                | Όχι        | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Αντικατάσταση για το "defaultTransport" στο EMAIL_CONFIG_PATH. Χρήσιμο για την ανάπτυξη του ίδιου αρχείου ρυθμίσεων σε διαφορετικά περιβάλλοντα.     | Όχι        | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | Το ID του λογαριασμού σας στο fastcomments.com. Χρησιμοποιείται για την εγγραφή του license key σας.                                             | Όχι        |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Ένα on-prem license key.                                                                                                                         | Όχι        |                                                       |
| GIPHY_API_KEY                  |                             | Giphy API Key. Αν δεν καθοριστεί, θα πρέπει να δημιουργήσετε έναν κανόνα ρύθμισης που να απενεργοποιεί τον επιλογέα gif.                            | Όχι        |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Χρησιμοποιείται για την ενσωμάτωση giphy. Μπορεί επίσης να αντικατασταθεί με κανόνες προσαρμογής του widget.                                     | Όχι        | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Χρησιμοποιείται για δυνατότητες με openai όπως προαιρετική ανίχνευση spam βασισμένη σε GPT.                                                       | Όχι        |                                                       |
| CDN_HOST_ADDR                  |                             | Το hostname από όπου θα προέρχονται τα assets. Προεπιλογή στην τιμή του HOSTNAME.                                                                 | Όχι        | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Το hostname από όπου θα ανακτώνται μεγάλα αρχεία (όπως exports). Προεπιλογή στην τιμή του CDN_HOST_ADDR.                                           | Όχι        | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Πού πρέπει να αποθηκεύονται μεγάλα αρχεία, όπως exports.                                                                                         | Όχι        | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Το hostname από όπου πρέπει να αποστέλλονται τα email.                                                                                           | Όχι        | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Το όνομα του cookie του fastcomments.                                                                                                            | Όχι        |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | Η τιμή του πεδίου "hostname" του cookie. Συνιστάται η προεπιλογή με τελεία στην αρχή.                                                              | Όχι        | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Χρησιμοποιείται για μεταφορτώσεις αρχείων χρηστών, avatar, κ.λπ. Προεπιλογή στο τοπικό FS αν δεν οριστεί.                                         | Όχι        |                                                       |
| S3_SECRET_KEY                  |                             | Χρησιμοποιείται για μεταφορτώσεις αρχείων χρηστών, avatar, κ.λπ.                                                                                 | Όχι        |                                                       |
| S3_REGION                      |                             | Χρησιμοποιείται για μεταφορτώσεις αρχείων χρηστών, avatar, κ.λπ.                                                                                 | Όχι        |                                                       |
| S3_BUCKET                      |                             | Χρησιμοποιείται για μεταφορτώσεις αρχείων χρηστών, avatar, κ.λπ.                                                                                 | Όχι        |                                                       |
| S3_HOST                        |                             | Χρησιμοποιείται για μεταφορτώσεις αρχείων χρηστών, avatar, κ.λπ.                                                                                 | Όχι        |                                                       |
| CACHE_DIR                      |                             | Τοποθεσία για αποθήκευση προαιρετικής offline cache, για όταν η DB δεν είναι διαθέσιμη. Περιηγείται περιοδικά με τα κορυφαία 100 θέματα σχολίων.     | Όχι        |                                                       |
| BACKUP_DIR                     |                             | Τοποθεσία για αποθήκευση δεδομένων για όταν η DB δεν είναι διαθέσιμη. Αν ένα σχόλιο υποβληθεί όταν η DB δεν είναι διαθέσιμη, πηγαίνει εδώ και επεξεργάζεται αργότερα. | Όχι        |                                                       |

Σημειώστε ότι όλες οι μεταβλητές σχετικές με domain χρησιμοποιούν την κατάληξη `_HOST` ή `_ADDR`. Η διαφορά είναι:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Η `EMAIL_CONFIG_PATH` πρέπει να περιέχει μια διαδρομή σε ένα αρχείο JSON με το ακόλουθο παράδειγμα μορφής:

[inline-code-attrs-start title = 'Διαμόρφωση Email'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Στο παραπάνω παράδειγμα ορίζουμε ένα προεπιλεγμένο `SMTP` email transport που ονομάζεται `mailgun`. Επίσης ορίζουμε ένα ειδικό transport που το χρησιμοποιούμε συγκεκριμένα για emails `@yahoo.com`. Σε ορισμένα σενάρια είναι επιθυμητό να χρησιμοποιηθεί συγκεκριμένος πάροχος ή IP αποστολής για ένα domain προκειμένου να βελτιστοποιηθεί η παράδοση. Αυτό είναι προαιρετικό.

### DocumentDB

Κατά τη σύνδεση με `DocumentDB` θα θελήσετε να ορίσετε `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` για συμβατότητα με τις προεπιλεγμένες ρυθμίσεις.

---