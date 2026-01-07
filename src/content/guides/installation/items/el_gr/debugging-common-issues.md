Εδώ είναι μερικά συμπτώματα που συναντάμε συχνά και κοινές λύσεις.

### Μήνυμα "This is a demo"

Αυτό εμφανίζεται όταν έχετε αντιγράψει τον κώδικα widget από την αρχική μας σελίδα, που χρησιμοποιεί τον demo
tenant μας. Για να χρησιμοποιήσετε τον δικό σας tenant, αντιγράψτε τον κώδικα widget από [εδώ](https://fastcomments.com/auth/my-account/get-acct-code).

### Σφάλμα "FastComments cannot load on this domain"

Το FastComments πρέπει να γνωρίζει ποια domains σας ανήκουν για να πιστοποιεί αιτήματα που σχετίζονται
με τον λογαριασμό σας. [Δείτε την τεκμηρίωσή μας](/guide-multiple-sites.html#add-domains-to-account) για να δείτε πώς
να επιλύσετε αυτό το σφάλμα (απλά προσθέστε το ακριβές subdomain + domain στον λογαριασμό σας).

Σημειώστε ότι αυτό θα πρέπει να συμβαίνει μόνο μετά τη λήξη της δοκιμαστικής περιόδου. Κατά τη διάρκεια της δοκιμαστικής περιόδου, τυχόν αιτήματα από νέα domains
θα προστίθενται αυτόματα στον λογαριασμό σας.

### Τα μεταφερμένα σχόλια δεν εμφανίζονται για προσαρμοσμένες εγκαταστάσεις

Συνήθως αυτό συμβαίνει όταν τα εισαγόμενα σχόλια είναι συνδεδεμένα με ένα `Page ID`, και εσείς περνάτε ένα URL
(ή καμία τιμή, οπότε χρησιμοποιεί το URL της σελίδας).

Μπορείτε να κάνετε debug αυτό [εξάγοντας τα σχόλιά σας](https://fastcomments.com/auth/my-account/manage-data/export) και βλέποντας τη στήλη `URL ID` (τρέχουσα στήλη `B`).

Βεβαιωθείτε ότι οι τιμές που βλέπετε στη στήλη `URL ID` είναι οι ίδιες τιμές που περνάτε στη διαμόρφωση του widget
ως παράμετρος `urlId`.

Για περαιτέρω εξήγηση, δοκιμάστε να διαβάσετε την [τεκμηρίωση Πώς τα σχόλια συνδέονται με σελίδες και άρθρα](/guide-customizations-and-configuration.html#url-id).

Αν όλα τα άλλα αποτύχουν, [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Το widget σχολίων δεν εμφανίζεται

Αν το widget σχολίων δεν εμφανίζεται, ελέγξτε την κονσόλα προγραμματιστή του Chrome για σφάλματα.

Για τις περισσότερες λανθασμένες ρυθμίσεις, το widget σχολίων θα εμφανίσει τουλάχιστον ένα σφάλμα στη σελίδα αν μπορεί
να φορτώσει. Το να μην βλέπετε τίποτα είναι συνήθως ένδειξη σφάλματος scripting.

### Η επιθυμητή διαμόρφωση δεν λειτουργεί όπως αναμένεται

Δοκιμάστε την [επέκταση Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) για να δείτε ποια
διαμόρφωση περνάει στο widget σχολίων. Αν όλα αποτύχουν, τραβήξτε ένα screenshot του τι λέει η επέκταση chrome
και [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Σχόλια που λείπουν στο ίδιο URL με διαφορετικό hash bang

Από προεπιλογή, το FastComments θα χρησιμοποιεί το URL της σελίδας για το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URL σας περιλαμβάνουν `#hashbangs`, και αυτά τα `#hashbangs`
δεν πρέπει να είναι μέρος του αναγνωριστικού που προσδιορίζει ένα νήμα σχολίων, μπορούμε απλά να αγνοήσουμε την τιμή hash bang, για παράδειγμα:

[inline-code-attrs-start title = 'Παράδειγμα αγνόησης hash bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Σημειώστε ότι μετά από αυτή την αλλαγή, θα πρέπει να γίνει migration για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Οι παράμετροι URL query επηρεάζουν το widget

Από προεπιλογή, το FastComments θα χρησιμοποιεί το URL της σελίδας για το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URL σας περιλαμβάνουν παραμέτρους query
που δεν πρέπει να είναι μέρος του αναγνωριστικού που προσδιορίζει ένα νήμα σχολίων, μπορούμε απλά να τις αγνοήσουμε, για παράδειγμα:

[inline-code-attrs-start title = 'Αγνόηση παραμέτρων query'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Σημειώστε ότι μετά από αυτή την αλλαγή, θα πρέπει να γίνει migration για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Δεν λαμβάνετε emails

Στο FastComments, καταβάλλουμε μεγάλη προσπάθεια για να διασφαλίσουμε ότι η παράδοση των email μας είναι όσο το δυνατόν πιο αξιόπιστη.
Ωστόσο, ορισμένοι πάροχοι email είναι διαβόητα δύσκολο να παραδώσουν αξιόπιστα. Ελέγξτε τον φάκελο spam σας
για μηνύματα από fastcomments.com.

Αν [επικοινωνήσετε μαζί μας](https://fastcomments.com/auth/my-account/help) μπορούμε συνήθως να παρέχουμε
περισσότερες πληροφορίες για το γιατί μπορεί να μην βλέπετε emails από εμάς.
