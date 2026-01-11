---
Για τοπική ανάπτυξη, χρησιμοποιήστε ένα εργαλείο όπως το [ngrok](https://ngrok.com/).

Για να απλοποιηθεί η διατήρηση της ασφάλειας του συστήματος, η τοπική ανάπτυξη ακολουθεί την ίδια διαδικασία με τη ρύθμιση και την ασφάλιση άλλων περιβαλλόντων. 

### Βήμα 1: Προσθέστε το "localhost" στους τομείς του λογαριασμού σας.

Προσθέστε το "localhost" [ως domain εδώ](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Βήμα 2: Επιλέξτε ένα API Key

Θα προσθέσουμε διαμόρφωση webhook για τον τομέα σας, οπότε θα χρειαστούμε ένα API key. [Μπορείτε να το κάνετε εδώ.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Στο "Associate with domain" - επιλέξτε τον "localhost" τομέα σας.

**ΣΗΜΕΙΩΣΗ: Εναλλακτικά, μπορείτε να χρησιμοποιήσετε ένα API Secret για όλη τη δραστηριότητα δοκιμών και τα staging περιβάλλοντα. Απλώς προσθέστε ένα API Secret για "All Domains", και δώστε του ένα όνομα όπως "test".**

Βεβαιωθείτε ότι έχετε ορίσει ένα API Secret για τους production τομείς σας. Τα γεγονότα για όλους τους άλλους τομείς θα χρησιμοποιούν το wildcard (testing) secret.

### Βήμα 3: Προσθέστε το Webhook σας

Ενώ τρέχετε το ngrok ή παρόμοιο εργαλείο, ορίστε την τιμή για το "localhost" [εδώ](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Όταν κάνετε κλικ στο `Send Test Payload`, θα στείλουμε δύο δοκιμαστικά γεγονότα για να ελέγξουμε ότι επικυρώνετε το API key.

Μόλις επικυρωθεί, πατήστε `Save`.

### Βήμα 4: Προσθέστε ένα Σχόλιο

Τώρα μπορείτε να προσθέσετε, επεξεργαστείτε ή διαγράψετε σχόλια και θα πρέπει να δείτε το σύστημά μας να καλεί το τοπικό σας μηχάνημα ανάπτυξης με τα γεγονότα, χρησιμοποιώντας το testing API key σας. Μπορεί να υπάρξει καθυστέρηση μέχρι 30 δευτερόλεπτα για να φτάσουν τα γεγονότα στο μηχάνημά σας.

---