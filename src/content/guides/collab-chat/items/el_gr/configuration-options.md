### Επισκόπηση

Το FastComments Collab Chat επεκτείνει το τυπικό widget σχολίων FastComments, οπότε κληρονομεί όλες τις επιλογές διαμόρφωσης από το βασικό widget, προσθέτοντας παράλληλα μερικές ειδικές για σχολιασμό κειμένου.

### Απαιτούμενη Διαμόρφωση

#### tenantId

Απαιτείται το Tenant ID του FastComments. Μπορείτε να το βρείτε στο [dashboard του FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Ειδικές Επιλογές του Collab Chat

#### urlId

Από προεπιλογή, το Collab Chat δημιουργεί ένα μοναδικό αναγνωριστικό για κάθε συνομιλία με βάση το URL της σελίδας, τη διαδρομή DOM του στοιχείου και το επιλεγμένο εύρος κειμένου. Μπορείτε να το παρακάμψετε με ένα προσαρμοσμένο `urlId`.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Αυτό είναι χρήσιμο όταν η δομή του URL σας ενδέχεται να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συνομιλίες, ή όταν θέλετε να μοιραστείτε σχολιασμούς σε πολλές σελίδες.

#### topBarTarget

Ελέγχει την εμφάνιση της πάνω μπάρας που δείχνει τον αριθμό χρηστών και συζητήσεων. Ορίστε `null` για να απενεργοποιήσετε εντελώς την πάνω μπάρα, ή δώστε ένα στοιχείο DOM για να εμφανιστεί σε συγκεκριμένη θέση.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Ενεργοποιήστε το στυλ σκοτεινής λειτουργίας όταν η σελίδα σας έχει σκούρο φόντο. Αυτή η ανίχνευση είναι αυτόματη, αλλά μπορεί να είναι επιθυμητό να την παρακάμψετε.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Μια συνάρτηση callback που ενεργοποιείται κάθε φορά που αλλάζει ο αριθμός των σχολίων. Αυτό είναι χρήσιμο για την ενημέρωση στοιχείων UI όπως σήματα ή τίτλων σελίδων.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Κληρονομημένες Επιλογές Διαμόρφωσης

Εφόσον το Collab Chat επεκτείνει το τυπικό widget σχολίων, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή διαμόρφωσης από το βασικό widget FastComments. Ακολουθούν μερικές συχνά χρησιμοποιούμενες επιλογές:

#### locale

Ορίστε τη γλώσσα του UI του widget. Το FastComments υποστηρίζει δεκάδες γλώσσες.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Κάντε όλες τις συνομιλίες μόνο για ανάγνωση. Οι χρήστες μπορούν να δουν υπάρχοντες σχολιασμούς αλλά δεν μπορούν να δημιουργήσουν νέους ή να απαντήσουν.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso και simpleSSO

Ενσωματώστε το σύστημα αυθεντικοποίησής σας χρησιμοποιώντας Single Sign-On.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Δείτε την τεκμηρίωση SSO για πλήρεις λεπτομέρειες σχετικά με τις επιλογές αυθεντικοποίησης.

#### maxReplyDepth

Ελέγξτε πόσα επίπεδα βάθους μπορούν να έχουν οι απαντήσεις. Από προεπιλογή, το Collab Chat το ορίζει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλευμένες απαντήσεις). Μπορείτε να το αλλάξετε αν θέλετε συζητήσεις με νήματα.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Εσωτερική Διαμόρφωση

Αυτές οι επιλογές ορίζονται αυτόματα από το Collab Chat και δεν πρέπει να παρακαμφθούν:

Το `productId` ορίζεται αυτόματα σε `3` για το Collab Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα του παραθύρου συνομιλίας. Το widget ανιχνεύει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει το UI ανάλογα.

### Πλήρες Παράδειγμα

Ακολουθεί ένα παράδειγμα που δείχνει πολλαπλές επιλογές διαμόρφωσης μαζί:

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Για μια πλήρη λίστα όλων των διαθέσιμων επιλογών διαμόρφωσης που κληρονομούνται από το βασικό widget, δείτε την κύρια τεκμηρίωση διαμόρφωσης του FastComments.