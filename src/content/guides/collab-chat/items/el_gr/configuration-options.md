### Επισκόπηση

Το FastComments Collab Chat επεκτείνει το πρότυπο widget σχολιασμών του FastComments, επομένως κληρονομεί όλες τις επιλογές διαμόρφωσης από το βασικό widget ενώ προσθέτει μερικές συγκεκριμένες για σχολιασμούς κειμένου.

### Απαραίτητη διαμόρφωση

#### tenantId

Απαιτείται το Tenant ID του FastComments. Μπορείτε να το βρείτε στον [πίνακα ελέγχου FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Ειδικές επιλογές Collab Chat

#### urlId

Από προεπιλογή, το Collab Chat δημιουργεί έναν μοναδικό αναγνωριστικό για κάθε συζήτηση με βάση το URL της σελίδας, τη διαδρομή DOM προς το στοιχείο και την επιλεγμένη περιοχή κειμένου. Μπορείτε να το αντικαταστήσετε με ένα προσαρμοσμένο `urlId`.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Αυτό είναι χρήσιμο όταν η δομή των URL μπορεί να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συζητήσεις, ή όταν θέλετε να μοιράζεστε σημειώσεις σε πολλές σελίδες.

#### topBarTarget

Ελέγχει την εμφάνιση της πάνω γραμμής που δείχνει τον αριθμό χρηστών και τον αριθμό συζητήσεων. Ορίστε σε `null` για να απενεργοποιήσετε πλήρως την πάνω γραμμή, ή παρέχετε ένα στοιχείο DOM για να την αποδώσετε σε συγκεκριμένη θέση.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Απενεργοποίηση πάνω γραμμής
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Εμφάνιση πάνω γραμμής σε προσαρμοσμένη θέση
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Ενεργοποιήστε το στυλ σκοτεινής εμφάνισης όταν η σελίδα σας έχει σκούρο φόντο. Αυτή η ανίχνευση γίνεται αυτόματα, αλλά μπορεί να είναι επιθυμητό να την παρακάμψετε.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Μια συνάρτηση επιστροφής κλήσης που εκτελείται κάθε φορά που αλλάζει ο αριθμός σχολίων. Αυτό είναι χρήσιμο για την ενημέρωση στοιχείων UI όπως badges ή τίτλους σελίδας.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Κληρονομημένες επιλογές διαμόρφωσης

Εφόσον το Collab Chat επεκτείνει το πρότυπο widget σχολιασμών, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή διαμόρφωσης από το βασικό widget του FastComments. Εδώ είναι μερικές συνήθως χρησιμοποιούμενες επιλογές:

#### locale

Ορίστε τη γλώσσα για το UI του widget. Το FastComments υποστηρίζει δεκάδες γλώσσες.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Ισπανικά
});
[inline-code-end]

#### readonly

Κάντε όλες τις συνομιλίες μόνο για ανάγνωση. Οι χρήστες μπορούν να βλέπουν υπάρχουσες σημειώσεις αλλά δεν μπορούν να δημιουργήσουν νέες ή να απαντήσουν.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Ενσωματώστε με το σύστημα αυθεντικοποίησής σας χρησιμοποιώντας Single Sign-On.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Διαμόρφωση SSO
    }
});
[inline-code-end]

Δείτε την τεκμηρίωση SSO για πλήρεις λεπτομέρειες σχετικά με τις επιλογές αυθεντικοποίησης.

#### maxReplyDepth

Ελέγξτε πόσα επίπεδα εμφώλευσης μπορούν να έχουν οι απαντήσεις. Από προεπιλογή, το Collab Chat το ορίζει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλευμένες απαντήσεις). Μπορείτε να το αλλάξετε αν θέλετε συνομιλίες με νήματα.

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Επιτρέψτε 3 επίπεδα εμφώλευσης
});
[inline-code-end]

### Εσωτερική διαμόρφωση

Αυτές οι επιλογές ορίζονται αυτόματα από το Collab Chat και δεν πρέπει να παρακαμφθούν:

Το `productId` ορίζεται αυτόματα σε `3` για το Collab Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα του παραθύρου συνομιλίας. Το widget ανιχνεύει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει το UI αναλόγως.

### Πλήρες παράδειγμα

Εδώ είναι ένα παράδειγμα που δείχνει πολλαπλές επιλογές διαμόρφωσης μαζί:

[inline-code-attrs-start title = "Παράδειγμα ρυθμίσεων"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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