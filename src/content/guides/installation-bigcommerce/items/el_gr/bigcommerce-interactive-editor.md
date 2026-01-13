---
Δεν συνιστάται να προσθέσετε το FastComments μέσω του Page Builder του BigCommerce, διότι τότε ο κώδικας πρέπει να προστεθεί χειροκίνητα σε κάθε σελίδα που θέλετε.

Ωστόσο, αν αυτό επιθυμείτε, πρέπει να χρησιμοποιήσετε το ακόλουθο απόσπασμα κώδικα. Αποσπάσματα κώδικα από άλλα σεμινάρια δεν θα λειτουργήσουν λόγω της φύσης του BigCommerce:

[inline-code-attrs-start title = 'Απόσπασμα για το Page Builder του BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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