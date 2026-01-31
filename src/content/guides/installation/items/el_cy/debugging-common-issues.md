Ακολουθούν κάποια συμπτώματα που συναντάμε συχνά, και οι συνήθεις λύσεις. 

### Μήνυμα "This is a demo"

Αυτό εμφανίζεται όταν έχετε αντιγράψει τον κώδικα του widget από την αρχική μας σελίδα, η οποία χρησιμοποιεί τον demo tenant μας. Για να χρησιμοποιήσετε τον δικό σας tenant, αντιγράψτε τον κώδικα του widget από [εδώ](https://fastcomments.com/auth/my-account/get-acct-code).

### Σφάλμα "FastComments cannot load on this domain"

Το FastComments χρειάζεται να γνωρίζει ποιες domains σας ανήκουν για να αυθεντικοποιήσει αιτήματα που σχετίζονται με τον λογαριασμό σας. [Δείτε την τεκμηρίωσή μας](/guide-multiple-sites.html#add-domains-to-account) για να δείτε πώς να επιλύσετε αυτό το σφάλμα (απλώς προσθέστε το ακριβές subdomain + domain στον λογαριασμό σας).

Σημειώστε ότι αυτό θα πρέπει να συμβεί μόνο αφού λήξει η δοκιμαστική περίοδος. Κατά τη διάρκεια της δοκιμαστικής περιόδου, οποιαδήποτε αιτήματα από νέες domains θα προστεθούν αυτόματα στον λογαριασμό σας.

### Migrated Comments Not Showing for Custom Installations

Συνήθως αυτό συμβαίνει όταν τα εισαχθέντα σχόλια έχουν συσχετιστεί με ένα `Page ID`, και εσείς περνάτε μια URL (ή καμία τιμή, οπότε προεπιλέγεται στην URL της σελίδας).

Μπορείτε να εντοπίσετε το πρόβλημα εξάγοντας τα σχόλιά σας μέσω του [exporting your comments](https://fastcomments.com/auth/my-account/manage-data/export) και βλέποντας τη στήλη `URL ID` (προς το παρόν Στήλη `B`).

Βεβαιωθείτε ότι οι τιμές που βλέπετε στη στήλη `URL ID` είναι οι ίδιες τιμές που περνάτε στη διαμόρφωση του widget ως παράμετρο `urlId`.

Για περαιτέρω εξήγηση, δοκιμάστε να διαβάσετε την τεκμηρίωσή μας [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Αν όλα τα άλλα αποτύχουν, [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Εάν το widget σχολίων δεν εμφανίζεται, ελέγξτε την κονσόλα του Chrome Developer για σφάλματα.

Σε περισσότερες περιπτώσεις λανθασμένης διαμόρφωσης, το widget σχολίων τουλάχιστον θα εμφανίσει ένα σφάλμα στη σελίδα εάν καταφέρει να φορτώσει. Το να μην βλέπετε τίποτα συνήθως υποδεικνύει σφάλμα scripting.

### Η επιθυμητή ρύθμιση δεν λειτουργεί όπως αναμένεται

Δοκιμάστε την [Chrome extension](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) μας για να δείτε ποια διαμόρφωση λαμβάνει το widget σχολίων. Αν όλα αποτύχουν, βγάλτε ένα στιγμιότυπο οθόνης (screenshot) αυτού που λέει το Chrome extension και [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Από προεπιλογή, το FastComments θα χρησιμοποιήσει την URL της σελίδας ως το "bucket" όπου αποθηκεύονται τα σχόλια. Εάν οι URL σας περιλαμβάνουν `#hashbangs`, και αυτά τα `#hashbangs` δεν θα έπρεπε να αποτελούν μέρος του αναγνωριστικού που ταυτοποιεί ένα νήμα σχολίων, μπορούμε απλά να αγνοήσουμε την τιμή hash bang, για παράδειγμα:

[inline-code-attrs-start title = 'Παράδειγμα αγνόησης Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Σημειώστε ότι μετά από αυτή την αλλαγή, θα χρειαστεί να γίνει μετεγκατάσταση για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Από προεπιλογή, το FastComments θα χρησιμοποιήσει την URL της σελίδας ως το "bucket" όπου αποθηκεύονται τα σχόλια. Εάν οι URL σας περιλαμβάνουν παραμέτρους ερωτήματος που δεν θα έπρεπε να αποτελούν μέρος του αναγνωριστικού που ταυτοποιεί ένα νήμα σχολίων, μπορούμε απλά να τις αγνοήσουμε, για παράδειγμα:

[inline-code-attrs-start title = 'Αγνόηση παραμέτρων ερωτήματος'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Σημειώστε ότι μετά από αυτή την αλλαγή, θα χρειαστεί να γίνει μετεγκατάσταση για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

Στο FastComments, καταβάλλουμε μεγάλη προσπάθεια για να εξασφαλίσουμε ότι η αποστολή των ηλεκτρονικών μηνυμάτων (emails) είναι όσο το δυνατόν πιο αξιόπιστη. Ωστόσο, κάποιοι πάροχοι email είναι διαβόητα δύσκολο να παραδίδονται με αξιοπιστία. Ελέγξτε τον φάκελο ανεπιθύμητων (spam) για μηνύματα από fastcomments.com.

Εάν [επικοινωνήσετε μαζί μας](https://fastcomments.com/auth/my-account/help), συνήθως μπορούμε να παρέχουμε περισσότερη πληροφόρηση για το γιατί ίσως να μην βλέπετε τα μηνύματα ηλεκτρονικού ταχυδρομείου από εμάς.

---