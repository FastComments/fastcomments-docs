### Επισκόπηση

Το FastComments Image Chat επεκτείνει το τυπικό widget σχολίων FastComments, οπότε κληρονομεί όλες τις επιλογές διαμόρφωσης από το βασικό widget, προσθέτοντας παράλληλα μερικές ειδικές για σχολιασμό εικόνων.

### Απαιτούμενη Διαμόρφωση

#### tenantId

Απαιτείται το Tenant ID του FastComments. Μπορείτε να το βρείτε στο [dashboard του FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Ειδικές Επιλογές του Image Chat

#### urlId

Από προεπιλογή, το Image Chat δημιουργεί ένα μοναδικό αναγνωριστικό για κάθε συνομιλία με βάση το URL της σελίδας, την πηγή της εικόνας και τις συντεταγμένες X/Y. Μπορείτε να το παρακάμψετε με ένα προσαρμοσμένο `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Αυτό είναι χρήσιμο όταν η δομή του URL σας ενδέχεται να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συνομιλίες, ή όταν θέλετε να μοιραστείτε σχολιασμούς σε πολλές σελίδες.

#### chatSquarePercentage

Ελέγχει το μέγεθος των κλικαρίστων δεικτών συνομιλίας ως ποσοστό του πλάτους της εικόνας. Η προεπιλογή είναι 5%, που σημαίνει ότι κάθε δείκτης είναι 5% του πλάτους της εικόνας.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Μεγαλύτεροι, πιο ορατοί δείκτες
});
```

Μικρότερες τιμές δημιουργούν λιγότερο ενοχλητικούς δείκτες που λειτουργούν καλύτερα για λεπτομερείς εικόνες. Μεγαλύτερες τιμές κάνουν τους δείκτες πιο εύκολα ορατούς και κλικαρίστους σε φορτωμένες εικόνες ή για χρήστες σε κινητές συσκευές.

#### hasDarkBackground

Ενεργοποιήστε το στυλ σκοτεινής λειτουργίας όταν η σελίδα σας έχει σκούρο φόντο.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Μια συνάρτηση callback που ενεργοποιείται κάθε φορά που αλλάζει ο αριθμός των σχολίων. Αυτό είναι χρήσιμο για την ενημέρωση στοιχείων UI όπως σήματα ή τίτλων σελίδων.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Κληρονομημένες Επιλογές Διαμόρφωσης

Εφόσον το Image Chat επεκτείνει το τυπικό widget σχολίων, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή διαμόρφωσης από το βασικό widget FastComments. Ακολουθούν μερικές συχνά χρησιμοποιούμενες επιλογές:

#### locale

Ορίστε τη γλώσσα του UI του widget. Το FastComments υποστηρίζει δεκάδες γλώσσες.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Ισπανικά
});
```

#### readonly

Κάντε όλες τις συνομιλίες μόνο για ανάγνωση. Οι χρήστες μπορούν να δουν υπάρχοντες δείκτες και συζητήσεις, αλλά δεν μπορούν να δημιουργήσουν νέους ή να απαντήσουν.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso και simpleSSO

Ενσωματώστε το σύστημα αυθεντικοποίησής σας χρησιμοποιώντας Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Διαμόρφωση SSO
    }
});
```

Δείτε την τεκμηρίωση SSO για πλήρεις λεπτομέρειες σχετικά με τις επιλογές αυθεντικοποίησης.

#### maxReplyDepth

Ελέγξτε πόσα επίπεδα βάθους μπορούν να φτάσουν οι απαντήσεις. Από προεπιλογή, το Image Chat το ορίζει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλευμένες απαντήσεις). Μπορείτε να το αλλάξετε αν θέλετε συζητήσεις με νήματα.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Επιτρέπει 3 επίπεδα εμφώλευσης
});
```

### Εσωτερική Διαμόρφωση

Αυτές οι επιλογές ορίζονται αυτόματα από το Image Chat και δεν πρέπει να παρακάμπτονται:

Το `productId` ορίζεται αυτόματα σε `2` για το Image Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα του παραθύρου συνομιλίας. Το widget ανιχνεύει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει το UI ανάλογα με παράθυρα συνομιλίας πλήρους οθόνης.

### Ευελιξία Στοιχείου-Στόχου

Η πρώτη παράμετρος του `FastCommentsImageChat` μπορεί να είναι είτε απευθείας ένα στοιχείο `<img>` είτε ένα στοιχείο-container με εικόνα μέσα του:

```javascript
// Απευθείας στοιχείο εικόνας
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container με εικόνα μέσα
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Το widget θα βρει την εικόνα αυτόματα αν περάσετε ένα στοιχείο-container.

### Πλήρες Παράδειγμα

Ακολουθεί ένα παράδειγμα που δείχνει πολλαπλές επιλογές διαμόρφωσης μαζί:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Η διαμόρφωση SSO σας
    },
    maxReplyDepth: 1
});
```

Για μια πλήρη λίστα όλων των διαθέσιμων επιλογών διαμόρφωσης που κληρονομούνται από το βασικό widget, δείτε την κύρια τεκμηρίωση διαμόρφωσης του FastComments.