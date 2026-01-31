Ακολουθούν κάποια συμπτώματα που συναντάμε συχνά, και κοινές λύσεις. 

### "This is a demo" Μήνυμα

Αυτό εμφανίζεται όταν έχετε αντιγράψει τον κώδικα του widget από την αρχική μας σελίδα, που χρησιμοποιεί τον demo
tenant μας. Για να χρησιμοποιήσετε τον δικό σας tenant, αντιγράψτε τον κώδικα του widget από [εδώ](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Σφάλμα

Το FastComments χρειάζεται να γνωρίζει ποιες domains σας ανήκουν για να αυθεντικοποιεί αιτήματα που σχετίζονται
με τον λογαριασμό σας. [Δείτε την τεκμηρίωσή μας](/guide-multiple-sites.html#add-domains-to-account) για να δείτε πώς
να επιλύσετε αυτό το σφάλμα (απλώς προσθέστε την ακριβή υπο-τοποθεσία + domain στον λογαριασμό σας).

Σημειώστε ότι αυτό θα πρέπει να εμφανίζεται μόνο αφού τελειώσει η δοκιμαστική περίοδος. Κατά τη διάρκεια της δοκιμαστικής περιόδου, οποιαδήποτε αιτήματα από νέες domains
θα προστίθενται αυτόματα στον λογαριασμό σας.

### Migrated Comments Not Showing for Custom Installations

Συνήθως αυτό συμβαίνει όταν τα εισαχθέντα σχόλια έχουν συνδεθεί με ένα `Page ID`, και εσείς περνάτε ένα URL
(ή καμία τιμή, οπότε χρησιμοποιείται ως προεπιλογή το URL της σελίδας).

Μπορείτε να εντοπίσετε το πρόβλημα κάνοντας [εξαγωγή των σχολίων σας](https://fastcomments.com/auth/my-account/manage-data/export) και βλέποντας την στήλη `URL ID` (επί του παρόντος Στήλη `B`).

Βεβαιωθείτε ότι οι τιμές που βλέπετε στη στήλη `URL ID` είναι οι ίδιες τιμές που περνάτε στη ρύθμιση του widget
ως παράμετρο `urlId`.

Για περαιτέρω εξήγηση, δοκιμάστε να διαβάσετε την [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id).

Αν όλα αποτύχουν, [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Αν το widget σχολίων δεν εμφανίζεται, ελέγξτε την κονσόλα προγραμματιστή του Chrome για σφάλματα.

Για τις περισσότερες δυσρυθμίσεις, το widget σχολίων τουλάχιστον θα εμφανίσει ένα σφάλμα στη σελίδα εάν είναι
σε θέση να φορτωθεί. Το να μην εμφανίζεται τίποτα συνήθως υποδηλώνει σφάλμα scripting.

### Desired Configuration Not Working as Expected

Δοκιμάστε την [επέκταση Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) για να δείτε ποια
ρύθμιση περνάει στο widget σχολίων. Αν όλα αποτύχουν, πάρτε ένα screenshot από ό,τι λέει η επέκταση Chrome
και [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Από προεπιλογή, το FastComments θα χρησιμοποιήσει το URL της σελίδας για το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URLs σας περιλαμβάνουν `#hashbangs`, και αυτά τα `#hashbangs`
δεν θα πρέπει να αποτελούν μέρος του αναγνωριστικού που προσδιορίζει ένα θέμα σχολίων, μπορούμε απλά να αγνοήσουμε την τιμή του hash bang, για παράδειγμα:

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

Σημειώστε ότι μετά από αυτή την αλλαγή, θα χρειαστεί να γίνει migration για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Από προεπιλογή, το FastComments θα χρησιμοποιήσει το URL της σελίδας για το "bucket" όπου αποθηκεύονται τα σχόλια. Αν τα URLs σας περιλαμβάνουν query parameters
που δεν θα πρέπει να αποτελούν μέρος του αναγνωριστικού που προσδιορίζει ένα θέμα σχολίων, μπορούμε απλά να τα αγνοήσουμε, για παράδειγμα:

[inline-code-attrs-start title = 'Παράδειγμα αγνόησης παραμέτρων ερωτήματος'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Σημειώστε ότι μετά από αυτή την αλλαγή, θα χρειαστεί να γίνει migration για τα υπάρχοντα σχόλια. [Για αυτό, επικοινωνήστε μαζί μας.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

Στο FastComments, δίνουμε πολλή δουλειά για να εξασφαλίσουμε ότι η παράδοση των email μας είναι όσο το δυνατόν πιο αξιόπιστη. Ωστόσο, μερικοί πάροχοι email είναι γνωστό ότι είναι δύσκολοι στην αξιόπιστη παράδοση. Ελέγξτε το φάκελο ανεπιθύμητων για μηνύματα από fastcomments.com.

Αν [επικοινωνήσετε μαζί μας](https://fastcomments.com/auth/my-account/help) συνήθως μπορούμε να δώσουμε
περισσότερες πληροφορίες σχετικά με το γιατί ίσως δεν βλέπετε μηνύματα από εμάς.