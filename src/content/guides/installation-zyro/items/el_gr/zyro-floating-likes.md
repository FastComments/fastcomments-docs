---
Το FastComments υποστηρίζει επίσης το widget Page Reacts (γνωστό και ως πλωτό κουμπί Like) για το Zyro.

Μπορείτε να το δείτε σε λειτουργία στο κάτω δεξί μέρος αυτής της σελίδας!

1. Πρώτα, πάρτε τον κώδικα:

[inline-code-attrs-start title = 'Κώδικας Floating Likes για Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Στη συνέχεια, στο Zyro, ανοίξτε τον δημιουργό του ιστότοπου.
3. Μεταβείτε στις Ρυθμίσεις ιστότοπου στο κάτω αριστερό μέρος.
4. Επιλέξτε Ενσωματώσεις.
5. Προσθέστε τον νέο κώδικα στο *τέλος* του πεδίου `Custom code`, και δημοσιεύστε τον ιστότοπό σας.
6. Δεν θα δείτε το widget σε λειτουργία προεπισκόπησης, αλλά θα εμφανιστεί στην δημοσιευμένη έκδοση του ιστότοπου.

---