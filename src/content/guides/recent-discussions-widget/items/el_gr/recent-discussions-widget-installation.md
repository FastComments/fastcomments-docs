Το Widget Πρόσφατων Συζητήσεων εμφανίζει σελίδες στον ιστότοπό σας που έχουν την πιο πρόσφατη δραστηριότητα σχολίων. Κάθε καταχώρηση εμφανίζει τον τίτλο της σελίδας, την ημερομηνία τελευταίας δραστηριότητας και το συνολικό πλήθος σχολίων. Ανιχνεύει αυτόματα σκοτεινά φόντα και προσαρμόζει το στυλ του ανάλογα.

## Βασική Εγκατάσταση

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

## Επιλογές Διαμόρφωσης

Η συνάρτηση `FastCommentsRecentDiscussionsV2` δέχεται τις ακόλουθες επιλογές διαμόρφωσης:

- **tenantId** (required): Το tenant ID του FastComments
- **count** (optional): Αριθμός σελίδων προς εμφάνιση. Η προεπιλογή είναι `20`, μέγιστο `100`
- **hasDarkBackground** (optional): Επιβάλλει στυλ σκοτεινής λειτουργίας. Αν δεν οριστεί, ανιχνεύεται αυτόματα από το φόντο της σελίδας

## Προχωρημένα Παραδείγματα

### Προσαρμοσμένος Αριθμός

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

### Επιβολή Σκοτεινής Λειτουργίας

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