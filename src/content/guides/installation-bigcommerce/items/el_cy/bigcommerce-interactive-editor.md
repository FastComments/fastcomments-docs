Δεν συνιστάται η προσθήκη του FastComments μέσω του Page Builder του BigCommerce, καθώς τότε ο κώδικας πρέπει να προστεθεί χειροκίνητα σε κάθε επιθυμητή σελίδα.

Ωστόσο, αν αυτό επιθυμείται, πρέπει να χρησιμοποιηθεί το ακόλουθο απόσπασμα κώδικα. Αποσπάσματα κώδικα από άλλα σεμινάρια δεν θα λειτουργήσουν λόγω της φύσης του BigCommerce:

[inline-code-attrs-start title = 'Απόσπασμα για το BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---