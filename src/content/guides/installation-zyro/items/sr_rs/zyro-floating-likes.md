FastComments такође подржава Page Reacts (такође познат као дугме за лебдећи лајк) видгет за Zyro.

Можете га видети у доњем десном углу ове странице!

1. Прво, преузмите код:

[inline-code-attrs-start title = 'Zyro код за лебдеће лајкове'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Затим, у Zyro-у, отворите алат за прављење сајта.
3. Идите на Подешавања сајта у доњем левом углу.
4. Изаберите Интеграције.
5. Додајте нови код на *крај* поља `Custom code`, и објавите ваш сајт.
6. Нећете видети видгет у режиму прегледа, али ће се појавити на објављеној верзији сајта.