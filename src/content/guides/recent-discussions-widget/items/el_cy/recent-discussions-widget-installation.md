Το Widget Πρόσφατων Συζητήσεων εμφανίζει σελίδες στον ιστότοπό σας που έχουν τη πιο πρόσφατη δραστηριότητα σχολίων. Κάθε εγγραφή εμφανίζει τον τίτλο της σελίδας, την ημερομηνία τελευταίας δραστηριότητας και το συνολικό πλήθος σχολίων. Εντοπίζει αυτόματα σκοτεινά φόντα και προσαρμόζει ανάλογα το στυλ του.

## Basic Installation

[inline-code-attrs-start title = 'Εγκατάσταση Widget Πρόσφατων Συζητήσεων'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuration Options

Η συνάρτηση `FastCommentsRecentDiscussionsV2` δέχεται τις ακόλουθες επιλογές διαμόρφωσης:

- **tenantId** (απαιτείται): Το tenant ID του λογαριασμού FastComments σας
- **count** (προαιρετικό): Αριθμός σελίδων προς εμφάνιση. Προεπιλογή είναι `20`, μέγιστο `100`
- **hasDarkBackground** (προαιρετικό): Επιβάλλει το στυλ σκοτεινής εμφάνισης. Αν δεν οριστεί, ανιχνεύεται αυτόματα από το φόντο της σελίδας

## Advanced Examples

### Custom Count

[inline-code-attrs-start title = 'Πρόσφατες Συζητήσεις με Προσαρμοσμένο Αριθμό'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Force Dark Mode

[inline-code-attrs-start title = 'Πρόσφατες Συζητήσεις με Σκοτεινή Λειτουργία'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---