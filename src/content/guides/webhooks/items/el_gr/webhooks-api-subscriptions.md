Webhooks μπορούν επίσης να διαχειρίζονται μέσω του REST API. Έτσι οι ενσωματώσεις όπως το Zapier εγγράφονται σε συμβάντα σχολίων χωρίς να αγγίζουν τον πίνακα ελέγχου, και ακολουθεί το πρότυπο REST Hooks: εγγραφή, λήψη συμβάντων, διαγραφή εγγραφής.

Οι συνδρομές API ζουν παράλληλα με τα webhooks που έχουν ρυθμιστεί στον πίνακα ελέγχου. Ένα συμβάν σχολίου παραδίδεται σε κάθε webhook που ταιριάζει με το domain του, το καθένα ως δική του παράδοση, ανεξάρτητα από το πώς δημιουργήθηκε το webhook.

## Authentication

Κάθε αίτημα χρειάζεται το API Key σας στην κεφαλίδα `x-api-key` (ή ως παράμετρο ερωτήματος `API_KEY`) και το tenant ID σας στην παράμετρο ερωτήματος `tenantId`. Και τα δύο εμφανίζονται στη σελίδα API Secret στον πίνακα ελέγχου.

## Subscribe

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Πεδίο | Απαιτείται | Περιγραφή |
|-------|------------|-----------|
| `url` | Ναι | Ένα απόλυτο URL http ή https. |
| `event` | Ναι | `comment-created`, `comment-updated` ή `comment-deleted`. |
| `domain` | Όχι | Ένα domain από τη ρύθμιση λογαριασμού σας. Προεπιλογή είναι `*`, που λαμβάνει συμβάντα για κάθε domain. |
| `method` | Όχι | `POST` (προεπιλογή), `PUT` ή `DELETE`. |

Η απάντηση περιέχει τη συνδρομή:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Η εγγραφή του ίδιου URL στο ίδιο συμβάν και domain ξανά επιστρέφει την υπάρχουσα συνδρομή αντί να δημιουργήσει διπλότυπο, ώστε ο πελάτης να μπορεί να επαναπροσπαθήσει με ασφάλεια. Κάθε tenant μπορεί να έχει έως 50 συνδρομές API.

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Επιστρέφει κάθε webhook για το tenant, συμπεριλαμβανομένων εκείνων που διαχειρίζονται στον πίνακα ελέγχου (`"source": "dashboard"`). Φιλτράρετε με `event`, `domain` ή `source`.

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Η διαγραφή μιας συνδρομής επίσης απορρίπτει τυχόν συμβάντα που είναι ακόμη στην ουρά της. Μόνο οι συνδρομές που δημιουργήθηκαν μέσω του API μπορούν να διαγραφούν με αυτόν τον τρόπο. Τα webhooks του πίνακα ελέγχου επεξεργάζονται στη σελίδα Webhooks.

## Payloads and signing

Οι παραδόσεις χρησιμοποιούν το ίδιο payload με τα webhooks του πίνακα ελέγχου (δείτε Data Structures) και υπογράφονται με το ίδιο σχήμα HMAC (δείτε Security & API Tokens). Οι συνδρομές API δεν λαμβάνουν ποτέ την παλιά κεφαλίδα `token`, επομένως επαληθεύστε την κεφαλίδα `X-FastComments-Signature` αντ' αυτού.

## Sample payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Επιστρέφει τα πιο πρόσφατα σχόλια του λογαριασμού ακριβώς στη μορφή που μεταφέρει μια παράδοση, ώστε μια ενσωμάτωση να μπορεί να εμφανίσει πραγματικά δείγματα δεδομένων πριν φτάσει το πρώτο συμβάν. Το `event` είναι προαιρετικό και μόνο επικυρώνεται, καθώς κάθε συμβάν παραδίδει το ίδιο αντικείμενο σχολίου. Το `limit` προεπιλογή είναι 3 και δέχεται τιμές από 1 έως 10. Κόστος 2 πιστώσεις API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Responding with 410 Gone

Αν το endpoint μιας συνδρομής API απαντήσει με HTTP `410 Gone`, το FastComments το θεωρεί ως διαγραφή εγγραφής: η συνδρομή διαγράφεται μαζί με τα ουράσμένα συμβάντα της, και δεν γίνονται περαιτέρω προσπάθειες παράδοσης. Τα webhooks που έχουν ρυθμιστεί στον πίνακα ελέγχου δεν διαγράφονται ποτέ αυτόματα· για αυτά το 410 είναι απλώς μια αποτυχία. Οποιοδήποτε άλλο σφάλμα επαναπροσπαθείται και τελικά απενεργοποιεί το webhook, όπως περιγράφεται στο How it Works & Handling Retries.

## Dashboard

Οι συνδρομές API εμφανίζονται στη λίστα Webhooks με την πηγή **API**, όπου ένας διαχειριστής μπορεί να τις επεξεργαστεί, να τις απενεργοποιήσει, να τις ενεργοποιήσει ξανά ή να τις διαγράψει.

---