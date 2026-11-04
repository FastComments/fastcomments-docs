Τα Webhooks μπορούν επίσης να διαχειριστούν μέσω του REST API. Έτσι γίνονται συνδρομές σε συμβάντα σχολίων από ενσωματώσεις όπως το Zapier χωρίς να αγγίζεται το dashboard, και ακολουθεί το μοτίβο REST Hooks: συνδρομή, λήψη συμβάντων, κατάργηση συνδρομής.

Οι συνδρομές μέσω API συνυπάρχουν με τα webhooks που ρυθμίζονται στο dashboard. Ένα συμβάν σχολίου παραδίδεται σε κάθε webhook που ταιριάζει με το domain του, το καθένα ως ξεχωριστή παράδοση, ανεξάρτητα από το πώς δημιουργήθηκε το webhook.

## Πιστοποίηση

Κάθε αίτημα απαιτεί το API Key σας στην επικεφαλίδα `x-api-key` (ή την παράμετρο `API_KEY` στο query string) και το tenant ID σας στην παράμετρο `tenantId` του query string. Και τα δύο εμφανίζονται στη σελίδα API Secret στο dashboard.

## Συνδρομή

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Πεδίο | Απαραίτητο | Περιγραφή |
|-------|----------|-------------|
| `url` | Ναι | Ένα απόλυτο URL http ή https. |
| `event` | Ναι | `comment-created`, `comment-updated` ή `comment-deleted`. |
| `domain` | Όχι | Ένα domain από τη ρύθμιση του λογαριασμού σας. Προεπιλογή είναι το `*`, το οποίο λαμβάνει συμβάντα για κάθε domain. |
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

Η επανάληψη συνδρομής του ίδιου URL στο ίδιο συμβάν και domain επιστρέφει την υπάρχουσα συνδρομή αντί να δημιουργήσει διπλότυπο, ώστε ένας client να μπορεί να επαναλάβει το αίτημα με ασφάλεια. Κάθε tenant μπορεί να έχει έως 50 συνδρομές μέσω API.

## Λίστα

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Επιστρέφει κάθε webhook για τον tenant, συμπεριλαμβανομένων εκείνων που διαχειρίζονται στο dashboard (`"source": "dashboard"`). Φιλτράρετε με `event`, `domain` ή `source`.

## Κατάργηση συνδρομής

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Η διαγραφή μιας συνδρομής απορρίπτει επίσης τυχόν συμβάντα που βρίσκονται ακόμα σε ουρά για αυτήν. Μόνο συνδρομές που δημιουργήθηκαν μέσω API μπορούν να διαγραφούν με αυτόν τον τρόπο· ένα webhook του dashboard, ή ένα id που δεν υπάρχει στον λογαριασμό σας, απαντά με `404` και κωδικό `not-found`. Τα webhooks του dashboard επεξεργάζονται στη σελίδα Webhooks.

## Payloads και υπογραφή

Οι παραδόσεις χρησιμοποιούν το ίδιο payload με τα webhooks του dashboard (βλ. Data Structures) και υπογράφονται με το ίδιο σχήμα HMAC (βλ. Security & API Tokens). Οι συνδρομές μέσω API δεν λαμβάνουν ποτέ την παλαιά επικεφαλίδα `token`, οπότε επαληθεύστε την επικεφαλίδα `X-FastComments-Signature` αντί αυτής.

## Δείγματα payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Επιστρέφει τα πιο πρόσφατα σχόλια του λογαριασμού ακριβώς στη μορφή που φέρει μια παράδοση, ώστε μια ενσωμάτωση να μπορεί να εμφανίσει πραγματικά δείγματα δεδομένων πριν φτάσει το πρώτο συμβάν. Το `event` είναι προαιρετικό και απλώς επικυρώνεται, αφού κάθε συμβάν παραδίδει το ίδιο αντικείμενο σχολίου. Το `limit` έχει προεπιλογή το 3 και δέχεται τιμές από 1 έως 10. Κοστίζει 2 API credits.

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

## Απάντηση με 410 Gone

Αν το endpoint μιας συνδρομής μέσω API απαντήσει με HTTP `410 Gone`, το FastComments το ερμηνεύει ως κατάργηση συνδρομής: η συνδρομή διαγράφεται μαζί με τα συμβάντα της σε ουρά και δεν γίνονται περαιτέρω προσπάθειες παράδοσης. Τα webhooks που ρυθμίζονται στο dashboard δεν διαγράφονται ποτέ αυτόματα· για αυτά ένα 410 είναι μια συνηθισμένη αποτυχία. Κάθε άλλη κατάσταση αποτυχίας επαναλαμβάνεται και τελικά απενεργοποιεί το webhook, όπως περιγράφεται στο How it Works & Handling Retries.

## Dashboard

Οι συνδρομές μέσω API εμφανίζονται στη λίστα Webhooks με πηγή **API**, όπου ένας διαχειριστής μπορεί να τις επεξεργαστεί, να τις απενεργοποιήσει, να τις επανενεργοποιήσει ή να τις διαγράψει.