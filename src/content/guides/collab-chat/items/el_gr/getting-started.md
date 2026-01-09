### Γρήγορη Εκκίνηση

Η έναρξη εργασίας με το Collab Chat είναι απλή. Χρειάζεστε το script του FastComments Collab Chat, ένα στοιχείο HTML που περιέχει το κείμενο που θέλετε να σχολιάσετε και ένα αντικείμενο διαμόρφωσης με το Tenant ID σας.

### Εγκατάσταση

Προσθέστε το script του Collab Chat στη σελίδα σας:

[inline-code-attrs-start title = 'Φόρτωση του script του Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Βασική Υλοποίηση

Εδώ ένα ελάχιστο παράδειγμα:

[inline-code-attrs-start title = 'Βασική Υλοποίηση Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Αντικαταστήστε το 'demo' με το πραγματικό Tenant ID της FastComments αν δεν είναι ήδη, το οποίο μπορείτε να βρείτε στον [πίνακα ελέγχου FastComments](https://fastcomments.com/auth/my-account/api-secret).

### Πώς λειτουργεί

Μόλις αρχικοποιηθεί, οι χρήστες μπορούν να επιλέξουν οποιοδήποτε κείμενο μέσα στο στοχευόμενο στοιχείο. Μετά από μια σύντομη καθυστέρηση (3.5 δευτερόλεπτα σε desktop), εμφανίζεται ένα προτροπικό παράθυρο που τους επιτρέπει να ξεκινήσουν μια συζήτηση. Όταν δημιουργηθεί μια συζήτηση, εμφανίζεται μια οπτική επισήμανση στο κείμενο. Άλλοι χρήστες μπορούν να τοποθετήσουν τον δείκτη πάνω στην επισήμανση ή να κάνουν κλικ σε αυτήν για να δουν και να συμμετάσχουν στη συζήτηση. Όλες οι συζητήσεις συγχρονίζονται σε πραγματικό χρόνο σε όλους τους επισκέπτες.

### Ζωντανή επίδειξη

Μπορείτε να δείτε το Collab Chat σε δράση στη [σελίδα ζωντανής επίδειξης](https://fastcomments.com/product/collab-chat).

### Επόμενα βήματα

Τώρα που έχετε τα βασικά να λειτουργούν, μπορείτε να προσαρμόσετε την εμφάνιση και τη συμπεριφορά στον οδηγό Επιλογών Διαμόρφωσης. Δείτε τον οδηγό Συμπεριφοράς Επιλογής Κειμένου για να καταλάβετε πώς λειτουργεί η επιλογή κειμένου. Μάθετε για το styling και την υποστήριξη dark mode στον οδηγό Προσαρμογής. Για προχωρημένες ενσωματώσεις, εξερευνήστε την Αναφορά API.

### Βιβλιοθήκες frontend

Όλες οι frontend βιβλιοθήκες της FastComments (react, vue, angular, κ.λπ.) έχουν Collab Chat.