FastComments такође подржава Page Reacts (познат и као Floating Like дугме) виџет за Hostinger.

Можете га видети у акцији у доњем десном углу ове странице!

### Напомена!

Ова упутства су за Hostinger Site Builder. Ако користите Hostinger *WordPress*, само преузмите код испод и додајте га на своју WordPress страницу
помоћу [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), који је бесплатан и једноставан додатак за додавање малих исечака кода на вашу страницу.

1. Прво, преузмите код:

[inline-code-attrs-start title = 'Hostinger Floating Likes код'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Затим, у Hostinger-у, отворите site builder.
3. Идите на Website Settings у доњем левом углу.
4. Изаберите Integrations.
5. Додајте нови код на *крај* поља `Custom code` и објавите своју страницу.
6. Виџет нећете видети у режиму прегледа, али ће се појавити на објављеној верзији странице.