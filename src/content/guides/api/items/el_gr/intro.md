### Το FastComments API

FastComments παρέχει ένα API για αλληλεπίδραση με πολλούς πόρους. Δημιουργήστε ενσωματώσεις με την πλατφόρμα μας, ή ακόμη και δημιουργήστε τους δικούς σας πελάτες!

Σε αυτήν την τεκμηρίωση, θα βρείτε όλους τους υποστηριζόμενους πόρους του API τεκμηριωμένους με τους τύπους αιτήσεων και απαντήσεων.

Για πελάτες Enterprise, όλη η πρόσβαση στο API καταγράφεται στο Αρχείο Ελέγχου.

### Δημιουργημένα SDKs

FastComments τώρα δημιουργεί ένα [API Spec](https://fastcomments.com/js/swagger.json) από τον κώδικά μας (αυτό δεν είναι ακόμη πλήρες, αλλά περιλαμβάνει πολλά APIs).

Έχουμε επίσης τώρα SDKs για δημοφιλείς γλώσσες:

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

### Έλεγχος Ταυτότητας

Το API πιστοποιείται με τη μεταβίβαση του [api key](https://fastcomments.com/auth/my-account/api-secret) είτε ως κεφαλίδα `X-API-KEY` είτε ως παράμετρο ερωτήματος `API_KEY`. Θα χρειαστείτε επίσης το `tenantId` για την πραγματοποίηση κλήσεων API. Αυτό μπορεί να ληφθεί από την ίδια σελίδα με το api key σας.

### Σημείωση Ασφαλείας

Αυτές οι διαδρομές προορίζονται να κληθούν από έναν **διακομιστή**. __ΜΗΝ__ τις καλέσετε από έναν περιηγητή. Κάνοντας αυτό θα εκθέσετε το κλειδί API σας - αυτό θα δώσει πλήρη πρόσβαση στον λογαριασμό σας σε όποιον μπορεί να δει τον πηγαίο κώδικα μιας σελίδας!

#### Επιλογή Ελέγχου Ταυτότητας 1 - Κεφαλίδες

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Επιλογή Ελέγχου Ταυτότητας 2 - Παράμετροι Ερωτήματος

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Επιλογή Ελέγχου Ταυτότητας 3 - Διακριτικό OAuth Bearer

- Header: `Authorization: Bearer fcat_...`

Οι εφαρμογές που συνδέονται μέσω του [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) λαμβάνουν ένα διακριτικό μέσω OAuth αντί για κλειδί API. Αυτό το διακριτικό λειτουργεί σε κάθε τελικό σημείο εδώ. Ο ενοικιαστής (tenant) υπονοείται από το διακριτικό, έτσι το `tenantId` είναι προαιρετικό, αλλά πρέπει να ταιριάζει με το διακριτικό όταν παρέχεται. Οι αιτήσεις `GET` χρειάζονται το πεδίο `read` και κάθε άλλη μέθοδος χρειάζεται το πεδίο `write`. Η ανακάλυψη ξεκινά στο `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Ανάγνωση των Δικών Σας Εγγραφών

FastComments παρέχει διαθεσιμότητα Active-Active. Τα αιτήματα από το κέντρο δεδομένων σας δρομολογούνται στο [πλησιέστερο σημείο παρουσίας](https://sophon.fastcomments.com/) με το δικό σας. Αυτό είναι αυτόματο, και συνήθως μπορείτε να παρατηρήσετε τη λογική ανάγνωσης-μετά-εγγραφής. Εάν θέλετε να είστε σίγουροι ότι διαβάζετε τις δικές σας εγγραφές, μπορείτε να συνδέσετε τα αιτήματά σας σε μια συγκεκριμένη περιοχή χρησιμοποιώντας αυτήν την περιοχή ως κεντρικό API (ωστόσο αυτό συνήθως δεν χρειάζεται για τις περισσότερες ενσωματώσεις):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Σημειώστε ότι αν το κάνετε αυτό, ίσως θέλετε να ορίσετε εναλλακτική λύση, καθώς έχουμε αποσύρει κόμβους εισόδου στο παρελθόν και χρησιμοποιούμε νέες ονομασίες για τη μετάβαση.

---