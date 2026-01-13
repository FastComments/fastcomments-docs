### Το API του FastComments

Το FastComments παρέχει ένα API για αλληλεπίδραση με πολλούς πόρους. Δημιουργήστε ενσωματώσεις με την πλατφόρμα μας, ή ακόμη και δημιουργήστε τους δικούς σας clients!

Σε αυτή την τεκμηρίωση, θα βρείτε όλους τους υποστηριζόμενους πόρους από το API τεκμηριωμένους με τους τύπους αιτήσεων και απαντήσεων τους.

Για πελάτες Enterprise, όλη η πρόσβαση στο API καταγράφεται στο Αρχείο Ελέγχου.

### Παραγόμενα SDKs

Το FastComments πλέον δημιουργεί μια [Περιγραφή API](https://fastcomments.com/js/swagger.json) από τον κώδικά μας (αυτό δεν είναι ακόμη πλήρες, αλλά περιλαμβάνει πολλά APIs).

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

### Αυθεντικοποίηση

Το API αυθεντικοποιείται περνώντας το [κλειδί API](https://fastcomments.com/auth/my-account/api-secret) είτε ως `X-API-KEY` header είτε ως παράμετρο ερωτήματος `API_KEY`. Θα χρειαστείτε επίσης το `tenantId` σας για να κάνετε κλήσεις στο API. Αυτό μπορεί να ανακτηθεί από την ίδια σελίδα με το κλειδί API σας.

### Σημείωση Ασφαλείας

Αυτές οι διαδρομές προορίζονται να κληθούν από έναν **server**. __ΜΗΝ__ τις καλείτε από έναν περιηγητή. Κάνοντας κάτι τέτοιο θα εκθέσετε το κλειδί API σας - αυτό θα παρέχει πλήρη πρόσβαση στο λογαριασμό σας σε οποιονδήποτε μπορεί να δει τον πηγαίο κώδικα μιας σελίδας!

#### Επιλογή Αυθεντικοποίησης Πρώτη - Κεφαλίδες

- Κεφαλίδα: `X-API-KEY`
- Κεφαλίδα: `X-TENANT-ID`

#### Επιλογή Αυθεντικοποίησης Δεύτερη - Παράμετροι Ερωτήματος

- Παράμετρος ερωτήματος: `API_KEY`
- Παράμετρος ερωτήματος: `tenantId`