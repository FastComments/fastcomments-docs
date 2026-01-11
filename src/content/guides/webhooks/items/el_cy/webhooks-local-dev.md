Για τοπική ανάπτυξη, χρησιμοποιήστε ένα εργαλείο όπως το [ngrok](https://ngrok.com/).

Για να απλοποιηθεί η διατήρηση της ασφάλειας του συστήματος, η τοπική ανάπτυξη ακολουθεί την ίδια διαδικασία με τη ρύθμιση και την ασφάλιση άλλων περιβαλλόντων. 

### Βήμα 1: Προσθέστε "localhost" στα domains στον λογαριασμό σας.

Προσθέστε το "localhost" [ως domain εδώ](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Βήμα 2: Επιλέξτε ένα API Key

Θα προσθέσουμε ρύθμιση webhook για το domain σας, οπότε θα χρειαστούμε ένα API Key. [Μπορείτε να το κάνετε εδώ.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Στο «Associate with domain» - επιλέξτε το domain «localhost».

**ΣΗΜΕΙΩΣΗ: Εναλλακτικά, μπορείτε να χρησιμοποιήσετε ένα API Secret για όλη τη δοκιμαστική δραστηριότητα και τα staging περιβάλλοντα. Απλά προσθέστε ένα API Secret για "All Domains", και δώστε του ένα όνομα όπως "test".**

Βεβαιωθείτε ότι έχετε ορίσει ένα API Secret για το/τα production domain(s) σας. Τα συμβάντα για όλα τα άλλα domains θα χρησιμοποιούν το wildcard (testing) secret.

### Βήμα 3: Προσθέστε το Webhook σας

Ενώ τρέχετε το ngrok ή παρόμοιο εργαλείο, ορίστε την τιμή για το «localhost» [εδώ](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Όταν κάνετε κλικ στο `Send Test Payload`, θα στείλουμε δύο δοκιμαστικά συμβάντα για να ελέγξουμε ότι επικυρώνεται το API key.

Μόλις επικυρωθεί, πατήστε `Save`.

### Βήμα 4: Προσθέστε ένα Σχόλιο

Τώρα μπορείτε να προσθέσετε, επεξεργαστείτε ή διαγράψετε σχόλια και θα πρέπει να δείτε να καλούμε τον τοπικό υπολογιστή ανάπτυξής σας με τα συμβάντα, χρησιμοποιώντας το testing API key σας. Μπορεί να υπάρχει καθυστέρηση έως και 30 δευτερόλεπτα
για να φτάσουν τα συμβάντα στη μηχανή σας.

---