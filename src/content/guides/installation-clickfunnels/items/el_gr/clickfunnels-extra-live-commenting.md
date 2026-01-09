Αν ψάχνετε για το απόσπασμα κώδικα του ClickFunnels για ζωντανό σχολιασμό, δοκιμάστε το εξής:

[inline-code-attrs-start title = 'Απόσπασμα ζωντανού σχολιασμού ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="min-width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // κάποιοι πάροχοι αλλάζουν το απόσπασμα κώδικα ώστε να είναι ασύγχρονο
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

---