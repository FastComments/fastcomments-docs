### Βασικό Παράδειγμα

Ο απλούστερος τρόπος χρήσης του Collab Chat είναι να στοχεύσετε ένα μόνο περιέκτη περιεχομένου. Αυτό το παράδειγμα δείχνει πώς να ενεργοποιήσετε τις σημειώσεις κειμένου σε ένα άρθρο:

[inline-code-attrs-start title = 'Βασικό Παράδειγμα Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Παράδειγμα με Προσαρμοσμένο URL ID (ανά σελίδα βιβλίου, άρθρο κ.λπ.)

Κατά προεπιλογή, το Collab Chat χρησιμοποιεί το URL της σελίδας σε συνδυασμό με τη διαδρομή του στοιχείου και το εύρος κειμένου για να αναγνωρίζει τις συζητήσεις. Μπορείτε να παρέχετε προσαρμοσμένο `urlId` για να έχετε μεγαλύτερο έλεγχο στον τρόπο ομαδοποίησης των συζητήσεων:

[inline-code-attrs-start title = 'Collab Chat με Προσαρμοσμένο URL ID'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

Αυτό είναι χρήσιμο εάν η δομή του URL σας αλλάζει αλλά θέλετε να διατηρήσετε τις ίδιες συζητήσεις, ή εάν θέλετε να μοιράζεστε τις ίδιες σημειώσεις συζήτησης σε πολλαπλές σελίδες.

### Παράδειγμα με Σκοτεινή Λειτουργία

Εάν ο ιστότοπός σας έχει σκούρο φόντο, ενεργοποιήστε τη υποστήριξη σκοτεινής λειτουργίας για να διασφαλίσετε ότι το UI του chat εμφανίζεται σωστά:

[inline-code-attrs-start title = 'Collab Chat με Σκοτεινή Λειτουργία'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - Dark Mode</title>
    <style>
        body {
            background-color: #1a1a1a !important;
            color: #e0e0e0 !important;
            font-family: system-ui, -apple-system, sans-serif;
            padding: 20px;
            margin: 0;
        }
        #article-content {
            max-width: 800px;
            margin: 0 auto;
            background-color: #2d2d2d;
            padding: 20px;
            border-radius: 8px;
        }
        h1 {
            color: #ffffff !important;
        }
        p {
            color: #e0e0e0 !important;
            line-height: 1.6;
        }
        .fastcomments-collab-chat-top-bar {
            background-color: #2d2d2d !important;
            color: #e0e0e0 !important;
            border-bottom: 1px solid #444 !important;
        }
    </style>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            hasDarkBackground: true
        });
    </script>
</body>
</html>
[inline-code-end]

### Παράδειγμα με Απενεργοποιημένη Πάνω Γραμμή

Κατά προεπιλογή, το Collab Chat εμφανίζει μια πάνω γραμμή με τον αριθμό χρηστών και τον αριθμό συζητήσεων. Μπορείτε να την απενεργοποιήσετε:

[inline-code-attrs-start title = 'Collab Chat με Απενεργοποιημένη Πάνω Γραμμή'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - No Top Bar</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            topBarTarget: null
        });
    </script>
</body>
</html>
[inline-code-end]

### Παράδειγμα με Callback Αριθμού Σχολίων

Μπορείτε να παρακολουθείτε πότε προστίθενται ή ενημερώνονται σχόλια χρησιμοποιώντας το callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Collab Chat με Callback Αριθμού Σχολίων'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Παράδειγμα με Πολλαπλές Ενότητες

Μπορείτε να αρχικοποιήσετε το Collab Chat σε πολλαπλές ξεχωριστές ενότητες της σελίδας σας. Κάθε ενότητα θα έχει τις δικές της ανεξάρτητες σημειώσεις:

[inline-code-attrs-start title = 'Collab Chat σε Πολλαπλές Ενότητες'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div id="intro-section">
    <h2>Introduction</h2>
    <p>Content for the introduction...</p>
</div>

<div id="main-section">
    <h2>Main Content</h2>
    <p>Content for the main article...</p>
</div>

<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    // Initialize on intro section
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // Initialize on main section
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]

---