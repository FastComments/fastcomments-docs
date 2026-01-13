Не се препоръчва да добавяте FastComments чрез Page Builder-а на BigCommerce, тъй като тогава кодът трябва да се добавя ръчно на всяка желана страница.

Въпреки това, ако това е желано, трябва да се използва следният фрагмент от код. Фрагменти от други уроци няма да работят поради естеството на BigCommerce:

[inline-code-attrs-start title = 'Фрагмент за Page Builder на BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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