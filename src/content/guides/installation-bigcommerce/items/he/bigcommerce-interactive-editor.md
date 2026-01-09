---
לא מומלץ להוסיף את FastComments באמצעות ה-Page Builder של BigCommerce מכיוון שבמקרה כזה יש להוסיף את הקוד ידנית לכל דף רצוי.

עם זאת, אם זה רצוי, יש להשתמש בקטע הקוד הבא. קטעי קוד ממדריכים אחרים לא יעבדו בשל אופיו של BigCommerce:

[inline-code-attrs-start title = 'קטע עבור Page Builder של BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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