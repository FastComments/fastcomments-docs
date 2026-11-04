Ακολουθούν ορισμένα συχνά συμπτώματα που συναντάμε και κοινές λύσεις.

### Μήνυμα "This is a demo"

Αυτό εμφανίζεται όταν έχετε αντιγράψει τον κώδικα του widget από την αρχική μας σελίδα, η οποία χρησιμοποιεί το demo tenant μας. Για να χρησιμοποιήσετε το δικό σας tenant, αντιγράψτε τον κώδικα του widget από [εδώ](https://fastcomments.com/auth/my-account/get-acct-code).

### Σφάλμα "FastComments cannot load on this domain"

Το FastComments χρειάζεται να γνωρίζει ποια domains σας ανήκουν για να πιστοποιεί τα αιτήματα που σχετίζονται με τον λογαριασμό σας. [Δείτε την τεκμηρίωσή μας](/guide-multiple-sites.html#add-domains-to-account) για να μάθετε πώς να επιλύσετε αυτό το σφάλμα (απλώς προσθέστε το ακριβές subdomain + domain στον λογαριασμό σας).

Σημειώστε ότι αυτό θα πρέπει να συμβεί μόνο μετά τη λήξη της δοκιμαστικής περιόδου. Κατά τη διάρκεια της δοκιμαστικής περιόδου, τυχόν αιτήματα από νέα domains θα προστεθούν αυτόματα στον λογαριασμό σας.

### Τα Μεταφερόμενα Σχόλια Δεν Εμφανίζονται σε Προσαρμοσμένες Εγκαταστάσεις

Συνήθως αυτό συμβαίνει όταν τα εισαγόμενα σχόλια είναι συνδεδεμένα με ένα `Page ID`, και εσείς περνάτε ένα URL (ή καμία τιμή, οπότε προεπιλέγεται το URL της σελίδας).

Μπορείτε να το αποσφαλματώσετε [εξάγοντας τα σχόλιά σας](https://fastcomments.com/auth/my-account/manage-data/export) και βλέποντας τη στήλη `URL ID` (προς το παρόν η στήλη `B`).

Βεβαιωθείτε ότι οι τιμές που βλέπετε στη στήλη `URL ID` είναι οι ίδιες τιμές που περνάτε στη ρύθμιση του widget ως παράμετρο `urlId`.

Για περαιτέρω εξήγηση, δοκιμάστε να διαβάσετε την τεκμηρίωσή μας [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Αν όλα τα άλλα αποτύχουν, [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Το Widget Σχολίων Δεν Εμφανίζεται

Αν το widget σχολίων δεν εμφανίζεται, ελέγξτε την κονσόλα προγραμματιστή του Chrome για σφάλματα.

Στις περισσότερες περιπτώσεις κακής ρύθμισης, το widget σχολίων θα εμφανίσει τουλάχιστον ένα σφάλμα στη σελίδα αν καταφέρει να φορτώσει. Το να μην βλέπετε τίποτα συνήθως σημαίνει σφάλμα scripting.

### Η Επιθυμητή Ρύθμιση Δεν Λειτουργεί όπως Αναμένεται

Δοκιμάστε τη [επέκταση Chrome](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) μας για να δείτε τι ρύθμιση περνάται στο widget σχολίων. Αν όλα αποτύχουν, τραβήξτε ένα screenshot από όσα λέει η επέκταση Chrome και [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Λείπουν Σχόλια στο Ίδιο URL με Διαφορετικό Hash Bang

Από προεπιλογή, το FastComments θα χρησιμοποιήσει το URL της σελίδας ως το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URL σας περιλαμβάνουν `#hashbangs`, και αυτά τα `#hashbangs` δεν θα πρέπει να αποτελούν μέρος του αναγνωριστικού που ταυτοποιεί ένα νήμα σχολίων, μπορούμε απλώς να αγνοήσουμε την τιμή του hash bang, για παράδειγμα:

[inline-code-attrs-start title = 'Παράδειγμα Αγνόησης Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Σημειώστε ότι μετά από αυτή την αλλαγή, θα πρέπει να γίνει μια μετάβαση για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Παράμετροι Query του URL Επηρεάζουν το Widget

Από προεπιλογή, το FastComments θα χρησιμοποιήσει το URL της σελίδας ως το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URL σας περιλαμβάνουν παραμέτρους query που δεν θα πρέπει να αποτελούν μέρος του αναγνωριστικού που ταυτοποιεί ένα νήμα σχολίων, μπορούμε απλώς να τις αγνοήσουμε, για παράδειγμα:

[inline-code-attrs-start title = 'Αγνόηση Παραμέτρων Query'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Σημειώστε ότι μετά από αυτή την αλλαγή, θα πρέπει να γίνει μια μετάβαση για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Δεν Λαμβάνονται Emails

Στη FastComments, καταβάλλουμε μεγάλη προσπάθεια ώστε η παράδοση των emails μας να είναι όσο το δυνατόν πιο αξιόπιστη. Ωστόσο, ορισμένοι πάροχοι email είναι διαβόητα δύσκολοι για αξιόπιστη παράδοση. Ελέγξτε τον φάκελο spam για μηνύματα από το fastcomments.com.

Αν [επικοινωνήσετε μαζί μας](https://fastcomments.com/auth/my-account/help), συνήθως μπορούμε να παρέχουμε περισσότερες πληροφορίες για το γιατί ενδέχεται να μη βλέπετε emails από εμάς.