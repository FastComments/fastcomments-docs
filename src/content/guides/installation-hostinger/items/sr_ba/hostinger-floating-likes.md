FastComments такође подржава Page Reacts (такође познат као плавајуће дугме 'Like') видгет за Hostinger.

Можете га видјети у доњем десном углу ове странице!

### Напомена!

Ове инструкције су за Hostinger Site Builder. Ако користите Hostinger *WordPress*, онда само узмите код испод и додајте га на ваш WordPress сајт користећи [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), који је бесплатан и једноставан додатак за додавање малих исјечака кода на ваш сајт.

1. Прво, узмите код:

[inline-code-attrs-start title = 'Код за плавајуће лајкове за Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Онда, у Hostinger-у, отворите Site Builder.
3. Идите у Website Settings у доњем левом углу.
4. Одаберите Integrations.
5. Додајте нови код на *крај* поља `Custom code`, и објавите ваш сајт.
6. Нећете видјети видгет у режиму прегледа, али ће се појавити на објављеној верзији сајта.