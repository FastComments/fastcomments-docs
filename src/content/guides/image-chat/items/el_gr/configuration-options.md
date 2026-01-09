### Overview

Το FastComments Image Chat επεκτείνει το βασικό widget σχολιασμών FastComments, οπότε κληρονομεί όλες τις επιλογές ρύθμισης από το βασικό widget ενώ προσθέτει μερικές ειδικές για σημειώσεις στην εικόνα.

### Required Configuration

#### tenantId

Απαιτείται το FastComments Tenant ID σας. Μπορείτε να το βρείτε στον [πίνακα ελέγχου FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Από προεπιλογή, το Image Chat δημιουργεί έναν μοναδικό αναγνωριστικό για κάθε συνομιλία βασισμένο στο URL της σελίδας, την πηγή της εικόνας και τις συντεταγμένες X/Y. Μπορείτε να το παρακάμψετε με ένα προσαρμοσμένο `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Αυτό είναι χρήσιμο όταν η δομή των URL σας μπορεί να αλλάξει αλλά θέλετε να διατηρήσετε τις ίδιες συνομιλίες, ή όταν θέλετε να μοιράζεστε τις σημειώσεις σε πολλές σελίδες.

#### chatSquarePercentage

Ελέγχει το μέγεθος των κλικ-επιλέξιμων δεικτών συνομιλίας ως ποσοστό του πλάτους της εικόνας. Η προεπιλογή είναι 5%, που σημαίνει ότι κάθε δείκτης είναι 5% του πλάτους της εικόνας.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Μεγαλύτεροι, πιο ορατοί δείκτες
});
```

Μικρότερες τιμές δημιουργούν λιγότερο ενοχλητικούς δείκτες που λειτουργούν καλύτερα για λεπτομερείς εικόνες. Μεγαλύτερες τιμές κάνουν τους δείκτες πιο εύκολους στην όραση και στο κλικ σε πολυάσχολες εικόνες ή για χρήστες σε κινητές συσκευές.

#### hasDarkBackground

Ενεργοποιήστε το στυλ σε σκούρο θέμα όταν η σελίδα σας έχει σκοτεινό φόντο.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Μια συνάρτηση callback που καλείται κάθε φορά που αλλάζει ο αριθμός σχολίων. Αυτό είναι χρήσιμο για την ενημέρωση στοιχείων UI όπως badges ή τίτλους σελίδας.

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

Καθώς το Image Chat επεκτείνει το πρότυπο widget σχολιασμών, μπορείτε να χρησιμοποιήσετε οποιαδήποτε επιλογή ρύθμισης από το βασικό widget FastComments. Εδώ είναι μερικές συνήθως χρησιμοποιούμενες επιλογές:

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

Ενσωματώστε με το σύστημά σας αυθεντικοποίησης χρησιμοποιώντας Single Sign-On.

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

Ελέγξτε πόσα επίπεδα βάθους μπορούν να έχουν οι απαντήσεις. Από προεπιλογή, το Image Chat το ορίζει σε 0, που σημαίνει ότι όλα τα σχόλια είναι επίπεδα (χωρίς εμφωλεύσεις). Μπορείτε να αλλάξετε αυτό αν θέλετε νηματοειδείς συνομιλίες.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Επιτρέπονται 3 επίπεδα εμφώλευσης
});
```

### Internal Configuration

Αυτές οι επιλογές ρυθμίζονται αυτόματα από το Image Chat και δεν πρέπει να παρακαμφθούν:

Το `productId` ορίζεται αυτόματα σε `2` για το Image Chat. Η επέκταση `floating-chat` φορτώνεται αυτόματα για να παρέχει τη λειτουργικότητα του παραθύρου συνομιλίας. Το widget εντοπίζει αυτόματα κινητές συσκευές (οθόνες κάτω από 768px πλάτος) και προσαρμόζει το UI ανάλογα με παράθυρα συνομιλίας σε πλήρη οθόνη.

### Target Element Flexibility

Η πρώτη παράμετρος του `FastCommentsImageChat` μπορεί να είναι είτε ένα στοιχείο `<img>` άμεσα είτε ένα στοιχείο container με μια εικόνα μέσα:

```javascript
// Άμεσο στοιχείο εικόνας
FastCommentsImageChat(document.getElementById('my-image'), config);

// Κοντέινερ με εικόνα μέσα
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Το widget θα βρει την εικόνα αυτόματα αν περάσετε ένα στοιχείο container.

### Complete Example

Εδώ είναι ένα παράδειγμα που δείχνει πολλές επιλογές ρύθμισης μαζί:

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

Για πλήρη λίστα όλων των διαθέσιμων επιλογών ρύθμισης που κληρονομούνται από το βασικό widget, δείτε την κύρια τεκμηρίωση ρυθμίσεων του FastComments.