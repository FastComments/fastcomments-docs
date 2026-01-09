Το FastComments υποστηρίζει επίσης το widget Page Reacts (γνωστό και ως πλωτό κουμπί Like) για το Hostinger.

Μπορείτε να το δείτε σε δράση στο κάτω δεξί μέρος αυτής της σελίδας!

### Σημείωση!

Οι παρακάτω οδηγίες αφορούν τον Hostinger Site Builder. Αν χρησιμοποιείτε Hostinger *WordPress*, τότε απλώς αντιγράψτε τον παρακάτω κώδικα και προσθέστε τον στην ιστοσελίδα WordPress σας
using [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), which is a free and easy plugin for adding small code snippets to your site.

1. Πρώτα, αντιγράψτε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας Πλωτού κουμπιού Like για Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Στη συνέχεια, στο Hostinger, ανοίξτε τον Site Builder.
3. Μεταβείτε στο Website Settings στο κάτω αριστερό μέρος.
4. Επιλέξτε Integrations.
5. Προσθέστε τον νέο κώδικα στο *τέλος* του `Custom code` πεδίου, και δημοσιεύστε την ιστοσελίδα σας.
6. Δεν θα δείτε το widget σε λειτουργία προεπισκόπησης, αλλά θα εμφανιστεί στην δημοσιευμένη έκδοση του ιστότοπου.