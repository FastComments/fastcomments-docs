---
Не рекомендуется добавлять FastComments через конструктор страниц BigCommerce, так как в этом случае код придётся вручную добавлять на каждую нужную страницу.

Однако, если это необходимо, следует использовать следующий фрагмент кода. Фрагменты кода из других руководств не будут работать из‑за особенностей BigCommerce:

[inline-code-attrs-start title = 'Фрагмент для конструктора страниц BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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