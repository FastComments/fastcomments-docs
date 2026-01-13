FastComments також підтримує віджет Page Reacts (також відомий як плаваюча кнопка «Подобається») для Hostinger.

Ви можете побачити його в дії в нижньому правому куті цієї сторінки!

### Примітка!

Ці інструкції призначені для конструктора сайтів Hostinger. Якщо ви використовуєте Hostinger *WordPress*, просто скопіюйте код нижче та додайте його на свій сайт WordPress за допомогою [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), який є безкоштовним і простим плагіном для додавання невеликих фрагментів коду на сайт.

1. Спочатку скопіюйте код:

[inline-code-attrs-start title = 'Код плаваючої кнопки «Подобається» для Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Далі в Hostinger відкрийте конструктор сайтів.
3. Перейдіть до Налаштування сайту у нижньому лівому куті.
4. Виберіть Інтеграції.
5. Додайте новий код у *кінець* поля `Custom code`, і опублікуйте свій сайт.
6. Ви не побачите віджет у режимі попереднього перегляду, але він з'явиться в опублікованій версії сайту.