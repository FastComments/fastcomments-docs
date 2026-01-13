### Το API του FastComments

Το FastComments παρέχει ένα API για αλληλεπίδραση με πολλούς πόρους. Δημιουργήστε ενσωματώσεις με την πλατφόρμα μας, ή ακόμα και να δημιουργήσετε τους δικούς σας πελάτες!

Σε αυτήν την τεκμηρίωση θα βρείτε όλους τους πόρους που υποστηρίζει το API, με τεκμηρίωση για τους τύπους των αιτημάτων και των απαντήσεων.

Για πελάτες Enterprise, όλη η πρόσβαση στο API καταγράφεται στο Αρχείο Ελέγχου.

### Δημιουργημένα SDKs

Το FastComments τώρα δημιουργεί μια [Προδιαγραφή API](https://fastcomments.com/js/swagger.json) από τον κώδικά μας (αυτό δεν είναι ακόμη πλήρες, αλλά περιλαμβάνει πολλά API).

Έχουμε επίσης πλέον SDKs για δημοφιλείς γλώσσες:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Πιστοποίηση

Η πιστοποίηση στο API γίνεται με την αποστολή του [κλειδιού API](https://fastcomments.com/auth/my-account/api-secret) είτε ως κεφαλίδα `X-API-KEY` είτε ως παράμετρος ερωτήματος `API_KEY`. Θα χρειαστείτε επίσης το `tenantId` για να κάνετε κλήσεις στο API. Αυτό μπορεί να ανακτηθεί από την ίδια σελίδα που βρίσκεται το κλειδί API.

### Σημείωση Ασφαλείας

Αυτές οι διαδρομές προορίζονται να κληθούν από έναν **διακομιστή**. __ΜΗΝ__ τις καλείτε από ένα πρόγραμμα περιήγησης. Αυτό θα εκθέσει το κλειδί API σας - θα παρέχει πλήρη πρόσβαση στον λογαριασμό σας σε οποιονδήποτε μπορεί να δει τον πηγαίο κώδικα μιας σελίδας!

#### Επιλογή Πιστοποίησης Πρώτη - Κεφαλίδες

- Κεφαλίδα: `X-API-KEY`
- Κεφαλίδα: `X-TENANT-ID`

#### Επιλογή Πιστοποίησης Δεύτερη - Παράμετροι Ερωτήματος

- Παράμετρος ερωτήματος: `API_KEY`
- Παράμετρος ερωτήματος: `tenantId`

---