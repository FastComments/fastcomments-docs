FastComments също поддържа уиджета Page Reacts (известен още като плаващ бутон „Харесвам“) за Hostinger.

Можете да го видите в действие в долния десен ъгъл на тази страница!

### Забележка!

Тези инструкции са за Hostinger Site Builder. Ако използвате Hostinger *WordPress*, просто вземете кода по-долу и го добавете към своя WordPress сайт
използвайки [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), който е безплатен и лесен плъгин за добавяне на малки фрагменти код към вашия сайт.

1. Първо, вземете кода:

[inline-code-attrs-start title = 'Код за плаващи харесвания на Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. След това в Hostinger отворете Site Builder.
3. Отидете в Настройки на уебсайта в долния ляв ъгъл.
4. Изберете Интеграции.
5. Добавете новия код в *края* на полето `Custom code`, и публикувайте сайта си.
6. Няма да видите уиджета в режим на предварителен преглед, но той ще се появи в публикуваната версия на сайта.