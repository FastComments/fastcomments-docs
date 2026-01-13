FastComments такође подржава Page Reacts (познат и као плутајуће дугме "Like") видџет за Hostinger.

Можете га видјети у акцији у доњем десном углу ове странице!

### Напомена!

Ова упутства су за Hostinger Site Builder. Ако користите Hostinger *WordPress*, онда само преузмите доњи код и додајте га на ваш WordPress сајт
using [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), који је бесплатан и једноставан додатак за додавање малих кодних исјечака на вашу страницу.

1. Прво, узмите код:

[inline-code-attrs-start title = 'Hostinger код за лебдеће лајкове'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Затим, у Hostinger-у, отворите Site Builder.
3. Идите на Website Settings у доњем лијевом углу.
4. Изаберите Integrations.
5. Додајте нови код на *крај* поља `Custom code`, и објавите ваш сајт.
6. Нећете видјети видџет у режиму прегледа, али ће се појавити на објављеној верзији сајта.

---