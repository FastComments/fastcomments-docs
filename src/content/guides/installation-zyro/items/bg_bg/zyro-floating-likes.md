FastComments също така поддържа уиджета Page Reacts (известен още като плаващ бутон "Харесва ми") за Zyro.

Можете да го видите в действие в долния десен ъгъл на тази страница!

1. Първо, вземете кода:

[inline-code-attrs-start title = 'Код за плаващи харесвания за Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. След това, в Zyro, отворете конструктора на сайта.
3. Отидете в Настройки на уебсайта в долния ляв ъгъл.
4. Изберете Интеграции.
5. Добавете новия код в *края* на полето `Custom code` и публикувайте сайта си.
6. Няма да видите уиджета в режим преглед, но той ще се появи в публикуваната версия на сайта.