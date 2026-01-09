### Γρήγορη Εκκίνηση

Η έναρξη με το Collab Chat είναι απλή. Χρειάζεστε το script του FastComments Collab Chat, ένα στοιχείο HTML που περιέχει το κείμενο που θέλετε να σχολιάσετε, και ένα αντικείμενο ρυθμίσεων με το Tenant ID σας.

### Εγκατάσταση

Προσθέστε το script του Collab Chat στη σελίδα σας:

[inline-code-attrs-start title = 'Φόρτωση του script του Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Βασική Υλοποίηση

Ορίστε ένα ελάχιστο παράδειγμα:

[inline-code-attrs-start title = 'Βασική υλοποίηση του Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Το κοντέινερ περιεχομένου σας -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Φόρτωση του script του Collab Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Αρχικοποίηση του Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Αντικαταστήστε `'demo'` με το πραγματικό Tenant ID του FastComments, εάν δεν το έχετε ήδη, το οποίο μπορείτε να βρείτε στο [πίνακα ελέγχου FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Πώς Λειτουργεί

Μόλις αρχικοποιηθεί, οι χρήστες μπορούν να επιλέξουν οποιοδήποτε κείμενο εντός του στοχευόμενου στοιχείου. Μετά από μια μικρή καθυστέρηση (3.5 δευτερόλεπτα σε desktop), εμφανίζεται μια προτροπή που τους επιτρέπει να ξεκινήσουν μια συζήτηση. Όταν δημιουργηθεί μια συζήτηση, εμφανίζεται οπτική επισήμανση στο κείμενο. Άλλοι χρήστες μπορούν να περάσουν το δείκτη πάνω από την επισήμανση ή να την κλικάρουν για να δουν και να συμμετάσχουν στη συζήτηση. Όλες οι συζητήσεις συγχρονίζονται σε πραγματικό χρόνο για όλους τους επισκέπτες.

### Ζωντανή Επίδειξη

Μπορείτε να δείτε το Collab Chat σε δράση στη [σελίδα ζωντανής επίδειξης](https://fastcomments.com/product/collab-chat).

### Επόμενα Βήματα

Τώρα που τα βασικά λειτουργούν, μπορείτε να προσαρμόσετε την εμφάνιση και τη συμπεριφορά στον οδηγό Επιλογών Διαμόρφωσης. Δείτε τον οδηγό Συμπεριφοράς Επιλογής Κειμένου για να κατανοήσετε πώς λειτουργεί η επιλογή κειμένου. Μάθετε για το styling και την υποστήριξη σκοτεινής λειτουργίας στον οδηγό Προσαρμογής. Για προχωρημένες ενσωματώσεις, εξερευνήστε την Αναφορά API.

### Βιβλιοθήκες frontend

Όλες οι frontend βιβλιοθήκες του FastComments (react, vue, angular, κ.λπ.) περιλαμβάνουν το Collab Chat.