Το FastComments υποστηρίζει επίσης το widget Page Reacts (aka κουμπί Floating Like) για το Hostinger.

Μπορείτε να το δείτε σε δράση στο κάτω δεξιά μέρος αυτής της σελίδας!

### Σημείωση!

Αυτές οι οδηγίες αφορούν το Hostinger Site Builder. Αν χρησιμοποιείτε το Hostinger *WordPress*, τότε απλώς πάρτε τον παρακάτω κώδικα και προσθέστε τον στον ιστότοπό σας στο WordPress
χρησιμοποιώντας το [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), ένα δωρεάν και εύκολο plugin για την προσθήκη μικρών τμημάτων κώδικα στον ιστότοπό σας.

1. Πρώτα, πάρτε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας Floating Likes για Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Στη συνέχεια, στο Hostinger, ανοίξτε το site builder.
3. Μεταβείτε στο Website Settings στο κάτω αριστερό μέρος.
4. Επιλέξτε Integrations.
5. Προσθέστε τον νέο κώδικα στο *τέλος* του πεδίου `Custom code` και δημοσιεύστε τον ιστότοπό σας.
6. Δεν θα δείτε το widget στη λειτουργία προεπισκόπησης, αλλά θα εμφανιστεί στη δημοσιευμένη έκδοση του ιστότοπου.