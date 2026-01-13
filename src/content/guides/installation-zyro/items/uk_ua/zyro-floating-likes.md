---
FastComments також підтримує віджет Page Reacts (також відомий як плаваюча кнопка «Вподобати») для Zyro.

Ви можете побачити його в дії у правому нижньому куті цієї сторінки!

1. Спершу скопіюйте код:

[inline-code-attrs-start title = 'Код плаваючих вподобань Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Далі в Zyro відкрийте конструктор сайту.
3. Перейдіть до Website Settings у нижньому лівому куті.
4. Виберіть Integrations.
5. Додайте новий код у *кінець* поля `Custom code`, і опублікуйте свій сайт.
6. Ви не побачите віджет у режимі попереднього перегляду, але він з'явиться в опублікованій версії сайту.

---