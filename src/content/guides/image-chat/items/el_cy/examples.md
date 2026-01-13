### Βασικό Παράδειγμα

Ο πιο απλός τρόπος για να χρησιμοποιήσετε το Image Chat είναι να στοχεύσετε ένα μόνο στοιχείο εικόνας. Αυτό το παράδειγμα δείχνει πώς να ενεργοποιήσετε διαδραστικές συζητήσεις πάνω σε μια εικόνα:

[inline-code-attrs-start title = 'Βασικό Παράδειγμα Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Παράδειγμα με Στοιχείο Περιέκτη

Μπορείτε επίσης να περάσετε ένα στοιχείο περιέκτη που έχει μια εικόνα μέσα του:

[inline-code-attrs-start title = 'Image Chat με Στοιχείο Περιέκτη'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Παράδειγμα με Προσαρμοσμένο Αναγνωριστικό URL

Από προεπιλογή, το Image Chat χρησιμοποιεί το URL της σελίδας σε συνδυασμό με την πηγή της εικόνας και τις συντεταγμένες για να αναγνωρίζει τις συζητήσεις. Μπορείτε να παρέχετε ένα προσαρμοσμένο `urlId`:

[inline-code-attrs-start title = 'Image Chat με Προσαρμοσμένο Αναγνωριστικό URL'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Αυτό είναι χρήσιμο αν η δομή των URL σας αλλάζει αλλά θέλετε να διατηρήσετε τις ίδιες συζητήσεις, ή αν θέλετε να μοιραστείτε τα ίδια σημεία συζήτησης σε πολλές σελίδες.

### Παράδειγμα με Σκοτεινή Λειτουργία

Αν ο ιστότοπός σας έχει σκούρο φόντο και το widget δεν το εντοπίζει αυτόματα όπως θα έπρεπε, μπορούμε χειροκίνητα να ενεργοποιήσουμε την υποστήριξη σκοτεινής λειτουργίας:

[inline-code-attrs-start title = 'Image Chat με Σκοτεινή Λειτουργία'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Παράδειγμα με Προσαρμοσμένο Μέγεθος Τετραγώνου Συνομιλίας

Μπορείτε να προσαρμόσετε το μέγεθος των κλικαριστών τετραγώνων που εμφανίζονται στην εικόνα. Το μέγεθος καθορίζεται ως ποσοστό του πλάτους της εικόνας:

[inline-code-attrs-start title = 'Image Chat με Προσαρμοσμένο Μέγεθος Τετραγώνου'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Μικρότερα τετράγωνα (το προεπιλεγμένο είναι 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Παράδειγμα με Callback Αρίθμησης Σχολίων

Παρακολουθήστε πότε προστίθενται ή ενημερώνονται σχόλια χρησιμοποιώντας το callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat με Callback Αρίθμησης Σχολίων'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Παράδειγμα με Πολλαπλές Εικόνες

Μπορείτε να αρχικοποιήσετε το Image Chat σε πολλές εικόνες. Κάθε εικόνα θα έχει τα δικά της ανεξάρτητα σημεία συζήτησης:

[inline-code-attrs-start title = 'Image Chat σε Πολλαπλές Εικόνες'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Αρχικοποίηση στην πρώτη εικόνα
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Αρχικοποίηση στη δεύτερη εικόνα
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---