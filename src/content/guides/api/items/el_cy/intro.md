### Το FastComments API

Το FastComments παρέχει ένα API για αλληλεπίδραση με πολλούς πόρους. Δημιουργήστε ενσωματώσεις με την πλατφόρμα μας ή ακόμη και τους δικούς σας clients!

Σε αυτήν την τεκμηρίωση, θα βρείτε όλους τους πόρους που υποστηρίζονται από το API τεκμηριωμένους με τους τύπους αιτήματος και απόκρισης.

Για πελάτες Enterprise, όλη η πρόσβαση στο API καταγράφεται στο Αρχείο Ελέγχου.

### Παραγόμενα SDKs

Το FastComments πλέον δημιουργεί μια [προδιαγραφή API](https://fastcomments.com/js/swagger.json) από τον κώδικά μας (αυτό δεν είναι ακόμα πλήρες, αλλά περιλαμβάνει πολλά API).

Επίσης τώρα έχουμε SDKs για δημοφιλείς γλώσσες:

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

Το API αυθεντικοποιείται περνώντας το [api key](https://fastcomments.com/auth/my-account/api-secret) είτε ως header `X-API-KEY` είτε ως query parameter `API_KEY`. Θα χρειαστείτε επίσης το `tenantId` για να πραγματοποιήσετε κλήσεις στο API. Αυτό μπορεί να ληφθεί από την ίδια σελίδα με το api key.

### Σημείωση Ασφαλείας

Αυτές οι διαδρομές προορίζονται να κληθούν από έναν **server**. __ΜΗΝ__ τις καλείτε από ένα πρόγραμμα περιήγησης. Το να το κάνετε αυτό θα εκθέσει το API key σας — αυτό θα παρέχει πλήρη πρόσβαση στον λογαριασμό σας σε οποιονδήποτε μπορεί να δει τον πηγαίο κώδικα μίας σελίδας!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Ανάγνωση των δικών σας εγγραφών

Το FastComments παρέχει Active-Active διαθεσιμότητα. Τα αιτήματα από το datacenter σας δρομολογούνται στο [κοντινότερο σημείο παρουσίας](https://sophon.fastcomments.com/) σε σχέση με εσάς. Αυτό γίνεται αυτόματα, και κανονικά μπορείτε να παρατηρήσετε συμπεριφορά τύπου "read-your-write". Αν θέλετε να βεβαιωθείτε ότι θα διαβάσετε τις δικές σας εγγραφές, μπορείτε να καθορίσετε τις αιτήσεις σας να χρησιμοποιούν συγκεκριμένη περιοχή ως host του API (παρόλα αυτά αυτό συνήθως δεν απαιτείται για τις περισσότερες ενσωματώσεις):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Σημειώστε ότι αν το κάνετε αυτό ίσως θέλετε να ορίσετε μια εναλλακτική επιλογή (fallback), καθώς στο παρελθόν έχουμε αποσύρει κόμβους εισόδου και έχουμε χρησιμοποιήσει νέα ονόματα για την αλλαγή.