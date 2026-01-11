Για τοπική ανάπτυξη, χρησιμοποιήστε ένα εργαλείο όπως το [ngrok](https://ngrok.com/).

Προκειμένου να απλοποιηθεί η διατήρηση του συστήματος ασφαλούς, η τοπική ανάπτυξη ακολουθεί την ίδια διαδικασία με τη ρύθμιση και την ασφάλιση άλλων περιβαλλόντων. 

### Step 1: Add "localhost" to domains in your account.

Προσθέστε το "localhost" [ως τομέα εδώ](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

Θα προσθέσουμε ρύθμιση webhook για τον τομέα σας, οπότε θα χρειαστούμε ένα κλειδί API. [Μπορείτε να το κάνετε εδώ.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**ΣΗΜΕΙΩΣΗ: Εναλλακτικά, μπορείτε να χρησιμοποιήσετε ένα API Secret για όλη τη δραστηριότητα δοκιμών και τα staging περιβάλλοντα. Απλά προσθέστε ένα API Secret για "All Domains", και δώστε του ένα όνομα όπως "test".**

Βεβαιωθείτε ότι έχετε ορίσει ένα API Secret για τους παραγωγικούς τομείς σας. Τα συμβάντα για όλους τους άλλους τομείς θα χρησιμοποιήσουν το wildcard (testing) secret.

### Step 3: Add Your Webhook

Ενώ τρέχετε το ngrok ή παρόμοιο εργαλείο, ορίστε την τιμή για το "localhost" [εδώ](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Μόλις επικυρωθεί, πατήστε `Save`.

### Step 4: Add A Comment

Τώρα μπορείτε να προσθέσετε, να επεξεργαστείτε ή να διαγράψετε σχόλια και θα πρέπει να δείτε να καλούμε τον τοπικό σας υπολογιστή ανάπτυξης με τα συμβάντα, χρησιμοποιώντας το δοκιμαστικό κλειδί API σας. Υπάρχει πιθανότητα να υπάρξει καθυστέρηση έως και 30 δευτερολέπτων
για να φτάσουν τα συμβάντα στη μηχανή σας.