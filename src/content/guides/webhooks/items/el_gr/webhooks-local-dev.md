Για τοπική ανάπτυξη, χρησιμοποιήστε ένα εργαλείο όπως το [ngrok](https://ngrok.com/).

Για να απλοποιηθεί η διατήρηση του συστήματος ασφαλούς, η τοπική ανάπτυξη ακολουθεί την ίδια διαδικασία με τη ρύθμιση και την ασφάλιση άλλων περιβαλλόντων.

### Βήμα 1: Προσθέστε το "localhost" στα domains στον λογαριασμό σας.

Προσθέστε το "localhost" [ως domain εδώ](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Η φόρμα προσθήκης domain στις ρυθμίσεις λογαριασμού με το localhost εισαγμένο στο πεδίο ονομάτων domain'; title='Προσθήκη localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Βήμα 2: Επιλέξτε ένα API Key

Θα προσθέσουμε ρύθμιση webhook για το domain σας, οπότε θα χρειαστούμε ένα API key. [Μπορείτε να το κάνετε εδώ.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Νέα φόρμα API secret με το συσχετισμένο domain ορισμένο σε localhost και το κλειδί ονομασμένο Testing'; title='Προσθήκη Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Στην ενότητα "Associate with domain" - επιλέξτε το domain "localhost" σας.

**ΣΗΜΕΙΩΣΗ: Εναλλακτικά, μπορείτε να χρησιμοποιήσετε ένα API Secret για όλες τις δοκιμαστικές δραστηριότητες και τα περιβάλλοντα staging. Απλώς προσθέστε ένα API Secret για "All Domains" και δώστε του ένα όνομα όπως "test".**

Βεβαιωθείτε ότι έχετε ορίσει ένα API Secret για τα production domain(s) σας. Τα γεγονότα για όλα τα άλλα domains θα χρησιμοποιούν το wildcard (testing) secret.

### Βήμα 3: Προσθέστε το Webhook Σας

Κατά τη λειτουργία του ngrok ή παρόμοιου εργαλείου, ορίστε την τιμή για το "localhost" [εδώ](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Διαχειριστής Webhooks με το domain localhost επιλεγμένο και ένα URL ngrok συμπληρωμένο στο endpoint δημιουργίας σχολίου'; title='Προσθήκη Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Κατά το κλικ στο `Send Test Payload`, θα στείλουμε δύο δοκιμαστικά γεγονότα για να ελέγξουμε ότι επικυρώνετε το API key.

Μόλις επικυρωθεί, πατήστε `Save`.

### Βήμα 4: Προσθέστε ένα Σχόλιο

Τώρα μπορείτε να προσθέσετε, επεξεργαστείτε ή διαγράψετε σχόλια και θα πρέπει να δείτε το σύστημα να καλεί το τοπικό σας μηχάνημα ανάπτυξης με τα γεγονότα, χρησιμοποιώντας το testing API key σας. Μπορεί να υπάρξει καθυστέρηση έως 30 δευτερόλεπτα για να φτάσουν τα γεγονότα στο μηχάνημά σας.

---