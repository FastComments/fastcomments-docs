---
### Το API του FastComments

Το FastComments παρέχει ένα API για αλληλεπίδραση με πολλούς πόρους. Δημιουργήστε ενσωματώσεις με την πλατφόρμα μας, ή ακόμη και δημιουργήστε τους δικούς σας πελάτες!

Σε αυτή την τεκμηρίωση θα βρείτε όλους τους υποστηριζόμενους πόρους από το API τεκμηριωμένους με τους τύπους αιτήσεων και αποκρίσεων τους.

Για πελάτες Enterprise, όλη η πρόσβαση στο API καταγράφεται στο Audit Log.

### Παραγόμενα SDK

Το FastComments τώρα δημιουργεί ένα [API Spec](https://fastcomments.com/js/swagger.json) από τον κώδικά μας (αυτό δεν είναι ακόμα πλήρες, αλλά περιλαμβάνει πολλά APIs).

Επίσης πλέον διαθέτουμε SDKs για δημοφιλείς γλώσσες:

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

Η πρόσβαση στο API πραγματοποιείται με την αποστολή του σας [api key](https://fastcomments.com/auth/my-account/api-secret) είτε ως header `X-API-KEY` είτε ως παραμετρος query `API_KEY`. Θα χρειαστείτε επίσης το `tenantId`
για την εκτέλεση κλήσεων API. Αυτό μπορεί να ανακτηθεί από την ίδια σελίδα με το api key.

### Σημείωση Ασφάλειας

Αυτές οι διαδρομές προορίζονται να καλούνται από έναν **διακομιστή**. __ΜΗΝ__ τις καλείτε από ένα πρόγραμμα περιήγησης. Κάτι τέτοιο θα εκθέσει το API key σας - αυτό θα παρέχει πλήρη πρόσβαση στο λογαριασμό σας
σε οποιονδήποτε μπορεί να δει τον πηγαίο κώδικα μιας σελίδας!

#### Επιλογή Πιστοποίησης Ένα - Κεφαλίδες

- Κεφαλίδα: `X-API-KEY`
- Κεφαλίδα: `X-TENANT-ID`

#### Επιλογή Πιστοποίησης Δύο - Παράμετροι Ερωτήματος

- Παράμετρος ερωτήματος: `API_KEY`
- Παράμετρος ερωτήματος: `tenantId`

### Ανάγνωση των δικών σας εγγραφών

Το FastComments παρέχει Active-Active διαθεσιμότητα. Τα αιτήματα από το data center σας δρομολογούνται στο [το κοντινότερο σημείο παρουσίας](https://sophon.fastcomments.com/) προς εσάς. Αυτό γίνεται αυτόματα, και συνήθως μπορείτε να παρατηρήσετε τη συμπεριφορά read-your-write. Εάν θέλετε να είστε σίγουροι ότι θα διαβάσετε τις δικές σας εγγραφές, μπορείτε να καρφιτσώσετε τα αιτήματά σας σε μια συγκεκριμένη περιοχή χρησιμοποιώντας εκείνη την περιοχή ως API host (ωστόσο αυτό συνήθως δεν είναι απαραίτητο για τις περισσότερες ενσωματώσεις):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Σημειώστε ότι αν το κάνετε αυτό ίσως θελήσετε να ορίσετε μια εφεδρική επιλογή, καθώς έχουμε απαρχαιώσει κόμβους εισόδου στο παρελθόν και χρησιμοποιούμε νέα ονόματα για την εναλλαγή.

---