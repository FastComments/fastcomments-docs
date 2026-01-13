### Επισκόπηση

Το FastComments Collab Chat επεκτείνει το τυπικό widget σχολιασμού του FastComments, οπότε κληρονομεί όλες τις επιλογές διαμόρφωσης από το βασικό widget ενώ προσθέτει μερικές επιλογές ειδικές για επισημάνσεις κειμένου.

### Απαιτούμενη Διαμόρφωση

#### tenantId

Απαιτείται το Tenant ID του FastComments σας. Μπορείτε να το βρείτε στον [πίνακα ελέγχου FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Επιλογές Ειδικές για το Collab Chat

#### urlId

Από προεπιλογή, το Collab Chat δημιουργεί ένα μοναδικό αναγνωριστικό για κάθε συνομιλία βασισμένο στο URL της σελίδας, στη διαδρομή DOM προς το στοιχείο και στο επιλεγμένο εύρος κειμένου. Μπορείτε να το παρακάμψετε με ένα προσαρμοσμένο `urlId`.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Αυτό είναι χρήσιμο όταν η δομή των URL σας μπορεί να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συνομιλίες, ή όταν θέλετε να μοιραστείτε επισημάνσεις μεταξύ πολλαπλών σελίδων.

#### topBarTarget

Ελέγχει την εμφάνιση της επάνω γραμμής που δείχνει τον αριθμό χρηστών και τον αριθμό συζητήσεων. Ορίστε σε `null` για να απενεργοποιήσετε πλήρως την επάνω γραμμή, ή δώστε ένα DOM στοιχείο για να την αποδώσετε σε μια συγκεκριμένη θέση.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Απενεργοποίηση επάνω γραμμής
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Απόδοση επάνω γραμμής σε προσαρμοσμένη θέση
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Ενεργοποιήστε το στυλ σκοτεινής λειτουργίας όταν η σελίδα σας έχει σκουρό φόντο. Αυτή η ανίχνευση γίνεται αυτόματα, αλλά ίσως θέλετε να την παρακάμψετε.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Συνάρτηση callback που εκτελείται κάθε φορά που αλλάζει ο αριθμός σχολίων. Χρήσιμο για ενημέρωση στοιχείων διεπαφής όπως badges ή τίτλοι σελίδων.

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

Επειδή το Collab Chat επεκτείνει το πρότυπο widget σχολιασμού, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή διαμόρφωσης από το βασικό widget FastComments. Ακολουθούν μερικές συνήθεις χρησιμοποιούμενες επιλογές:

#### locale

Ορίστε τη γλώσσα για τη διεπαφή του widget. Το FastComments υποστηρίζει δεκάδες γλώσσες.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Ισπανικά
});
[inline-code-end]

#### readonly

Κάντε όλες τις συνομιλίες μόνο για ανάγνωση. Οι χρήστες μπορούν να δουν υπάρχουσες επισημάνσεις αλλά δεν μπορούν να δημιουργήσουν νέες ή να απαντήσουν.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Ενσωματώστε με το σύστημα πιστοποίησής σας χρησιμοποιώντας Single Sign-On.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Ρυθμίσεις SSO
    }
});
[inline-code-end]

Δείτε την τεκμηρίωση SSO για πλήρεις λεπτομέρειες σχετικά με τις επιλογές πιστοποίησης.

#### maxReplyDepth

Ελέγξτε πόσα επίπεδα βάθους μπορούν να έχουν οι απαντήσεις. Από προεπιλογή, το Collab Chat το ορίζει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλευμένες απαντήσεις). Μπορείτε να αλλάξετε αυτό εάν θέλετε νηματικές συνομιλίες.

[inline-code-attrs-start title = "Παράδειγμα Διαμόρφωσης"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Επιτρέπονται 3 επίπεδα ένθεσης
});
[inline-code-end]

### Εσωτερική Διαμόρφωση

Αυτές οι επιλογές ορίζονται αυτόματα από το Collab Chat και δεν πρέπει να παρακαμφθούν:

Το `productId` ορίζεται αυτόματα σε `3` για το Collab Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα παραθύρου συνομιλίας. Το widget εντοπίζει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει τη διεπαφή ανάλογα.

### Πλήρες Παράδειγμα

Ακολουθεί ένα παράδειγμα που δείχνει πολλές επιλογές διαμόρφωσης μαζί:

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
        // Η ρύθμιση SSO σας
    },
    maxReplyDepth: 1
});
[inline-code-end]

Για μια πλήρη λίστα όλων των διαθέσιμων επιλογών διαμόρφωσης που κληρονομούνται από το βασικό widget, δείτε την κύρια τεκμηρίωση διαμόρφωσης του FastComments.