### Overview

Το FastComments Image Chat επεκτείνει το βασικό widget σχολιασμού του FastComments, οπότε κληρονομεί όλες τις επιλογές διαμόρφωσης από το βασικό widget ενώ προσθέτει μερικές ειδικές για τις σημειώσεις πάνω σε εικόνες.

### Required Configuration

#### tenantId

Απαιτείται το Tenant ID του FastComments. Μπορείτε να το βρείτε στον [πίνακα ελέγχου του FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Από προεπιλογή, το Image Chat δημιουργεί ένα μοναδικό αναγνωριστικό για κάθε συνομιλία βασισμένο στο URL της σελίδας, στην πηγή της εικόνας, και στις συντεταγμένες X/Y. Μπορείτε να το αντικαταστήσετε με ένα προσαρμοσμένο `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Αυτό είναι χρήσιμο όταν η δομή των URL σας μπορεί να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συνομιλίες, ή όταν θέλετε να μοιράζεστε τις σημειώσεις ανάμεσα σε πολλαπλές σελίδες.

#### chatSquarePercentage

Ελέγχει το μέγεθος των σημείων συνομιλίας (clickable chat markers) ως ποσοστό του πλάτους της εικόνας. Η προεπιλογή είναι 5%, που σημαίνει ότι κάθε δείκτης είναι 5% του πλάτους της εικόνας.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Μεγαλύτεροι, πιο εμφανείς δείκτες
});
```

Μικρότερες τιμές δημιουργούν λιγότερο παρεμβατικούς δείκτες που λειτουργούν καλύτερα για λεπτομερείς εικόνες. Μεγαλύτερες τιμές κάνουν τους δείκτες πιο εύκολους στο να τους δει και να τους πατήσει ο χρήστης σε πολυσύχναστες εικόνες ή για χρήστες σε κινητές συσκευές.

#### hasDarkBackground

Ενεργοποιεί το στυλ dark mode όταν η σελίδα σας έχει σκούρο φόντο.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Μια συνάρτηση callback που εκτελείται όποτε αλλάζει ο αριθμός σχολίων. Αυτό είναι χρήσιμο για την ενημέρωση στοιχείων UI όπως badges ή τίτλους σελίδας.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Εφόσον το Image Chat επεκτείνει το βασικό widget σχολιασμού, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή διαμόρφωσης από το βασικό widget του FastComments. Εδώ είναι μερικές κοινά χρησιμοποιούμενες επιλογές:

#### locale

Ορίστε τη γλώσσα για το UI του widget. Το FastComments υποστηρίζει δεκάδες γλώσσες.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Ισπανικά
});
```

#### readonly

Κάντε όλες τις συνομιλίες μόνο για ανάγνωση. Οι χρήστες μπορούν να δουν υπάρχοντες δείκτες και συζητήσεις αλλά δεν μπορούν να δημιουργήσουν νέες ή να απαντήσουν.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Ενσωματώστε με το σύστημα αυθεντικοποίησής σας χρησιμοποιώντας Single Sign-On.

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

Ελέγξτε πόσα επίπεδα εμφώλευσης μπορούν να έχουν οι απαντήσεις. Από προεπιλογή, το Image Chat το θέτει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλευμένες απαντήσεις). Μπορείτε να το αλλάξετε αν θέλετε συζητήσεις με νήματα.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Επιτρέπει 3 επίπεδα εμφωλευμένων απαντήσεων
});
```

### Internal Configuration

Αυτές οι επιλογές ορίζονται αυτόματα από το Image Chat και δεν πρέπει να αντικαθίστανται:

Το `productId` ορίζεται αυτόματα σε `2` για το Image Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα παραθύρου συνομιλίας. Το widget ανιχνεύει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει το UI ανάλογα με πλήρη οθόνη για τα παράθυρα συνομιλίας.

### Target Element Flexibility

Η πρώτη παράμετρος στο `FastCommentsImageChat` μπορεί να είναι είτε ένα στοιχείο `<img>` απευθείας είτε ένα κοντέινερ με μια εικόνα μέσα:

```javascript
// Άμεσο στοιχείο εικόνας
FastCommentsImageChat(document.getElementById('my-image'), config);

// Κοντέινερ με εικόνα μέσα
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Το widget θα βρει την εικόνα αυτόματα αν περάσετε ένα στοιχείο κοντέινερ.

### Complete Example

Εδώ είναι ένα παράδειγμα που δείχνει πολλαπλές επιλογές διαμόρφωσης μαζί:

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