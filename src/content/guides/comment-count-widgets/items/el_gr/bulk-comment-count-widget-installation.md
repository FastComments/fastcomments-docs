Το Bulk Comment Count Widget ειναι σχεδιασμενο για την αποτελεσματικη εμφανιση του αριθμου σχολιων για πολλαπλες σελιδες στην ιδια σελιδα. Αντι να κανει μεμονωμενες κλησεις API για καθε αριθμο σχολιων, αυτο το widget ομαδοποιει τα αιτηματα για βελτιστη αποδοση.

## Βασικη Εγκατασταση

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Πως Λειτουργει

Το bulk widget λειτουργει:

1. Σαρωνοντας τη σελιδα για στοιχεια με την κλαση `fast-comments-count`
2. Διαβαζοντας το χαρακτηριστικο `data-fast-comments-url-id` απο καθε στοιχειο
3. Ομαδοποιωντας αιτηματα API για αποτελεσματικη ανακτηση πολλαπλων αριθμων σχολιων
4. Ενημερωνοντας καθε στοιχειο με τον καταλληλο αριθμο σχολιων

## Επιλογες Διαμορφωσης

Η συναρτηση `FastCommentsCommentCountBulk` δεχεται τις ακολουθες επιλογες διαμορφωσης:

- **tenantId** (απαιτειται): Το FastComments tenant ID σας
- **apiHost** (προαιρετικο): Προσαρμοσμενος API host αν χρησιμοποιειτε μια αυτο-φιλοξενουμενη εγκατασταση

## Παραδειγμα απο τον Πραγματικο Κοσμο

Εδω ειναι ενα πρακτικο παραδειγμα που δειχνει πως μπορειτε να χρησιμοποιησετε το bulk widget σε μια λιστα αναρτησεων blog:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Ζητηματα Αποδοσης

Το bulk widget βελτιστοποιει αυτοματα την αποδοση με:

- **Ομαδοποιηση αιτηματων**: Πολλαπλοι αριθμοι σχολιων ανακτωνται σε μια μονο κληση API
- **Ορια μεγεθους αιτηματων**: Τα αιτηματα διαχωριζονται αυτοματα αν η λιστα URL γινει πολυ μεγαλη (πανω απο 1.000 χαρακτηρες)
- **Αποδιπλασιασμος**: Πολλαπλα στοιχεια με το ιδιο `data-fast-comments-url-id` μοιραζονται τον ιδιο αριθμο

## Πολλαπλα Στοιχεια με το Ιδιο URL ID

Μπορειτε να εχετε πολλαπλα στοιχεια στη σελιδα με το ιδιο `data-fast-comments-url-id`. Ολα θα ενημερωθουν με τον ιδιο αριθμο:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Τοπικοποιηση

Το bulk widget μορφοποιει αυτοματα τους αριθμους σχολιων βασει των ρυθμισεων γλωσσας του FastComments. Παρεχει καταλληλο κειμενο για:

- Μηδεν σχολια
- Ενα σχολιο
- Πολλαπλα σχολια

## Ποτε να Χρησιμοποιησετε Bulk εναντι Single Widget

**Χρησιμοποιηστε το Bulk Widget οταν:**
- Εχετε πολλαπλους αριθμους σχολιων στην ιδια σελιδα
- Εμφανιζετε μια λιστα αναρτησεων/αρθρων με αριθμους σχολιων
- Η αποδοση ειναι σημαντικη (μειωνει τις κλησεις API)

**Χρησιμοποιηστε το Single Widget οταν:**
- Χρειαζεστε μονο εναν αριθμο σχολιων στη σελιδα
- Χρειαζεστε ζωντανες ενημερωσεις (το single widget υποστηριζει ενημερωσεις σε πραγματικο χρονο)
- Θελετε περισσοτερο ελεγχο στη συμπεριφορα του μεμονωμενου widget
