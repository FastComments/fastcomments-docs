---
Не рекомендується додавати FastComments через Page Builder BigCommerce, оскільки в такому разі код доведеться вручну додавати на кожну потрібну сторінку.

Однак, якщо це бажано, слід використовувати наведену нижче частину коду. Фрагменти коду з інших підручників не працюватимуть через особливості BigCommerce:

[inline-code-attrs-start title = 'Фрагмент для конструктора сторінок BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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