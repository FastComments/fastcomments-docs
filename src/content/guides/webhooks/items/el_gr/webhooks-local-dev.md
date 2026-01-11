---
Για τοπική ανάπτυξη, χρησιμοποιήστε ένα εργαλείο όπως το [ngrok](https://ngrok.com/).

Για να απλοποιηθεί η διατήρηση της ασφάλειας του συστήματος, η τοπική ανάπτυξη ακολουθεί την ίδια διαδικασία που χρησιμοποιείται για τη ρύθμιση και την ασφάλιση άλλων περιβαλλόντων. 

### Βήμα 1: Προσθέστε το "localhost" στους τομείς του λογαριασμού σας.

Προσθέστε το "localhost" [ως τομέα εδώ](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Βήμα 2: Επιλέξτε ένα API Key

Θα προσθέσουμε ρύθμιση webhook για τον τομέα σας, οπότε θα χρειαστούμε ένα API key. [Μπορείτε να το κάνετε εδώ.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Βήμα 3: Προσθέστε το Webhook σας

Ενώ τρέχετε το ngrok ή παρόμοιο εργαλείο, ορίστε την τιμή για το "localhost" [εδώ](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Βήμα 4: Προσθέστε ένα σχόλιο

Τώρα μπορείτε να προσθέσετε, να επεξεργαστείτε ή να διαγράψετε σχόλια και θα πρέπει να δείτε να καλούμε τη τοπική σας μηχανή ανάπτυξης με τα συμβάντα, χρησιμοποιώντας το testing API key σας. Μπορεί να υπάρξει καθυστέρηση έως 30 δευτερόλεπτα
for the events to reach your machine.

---