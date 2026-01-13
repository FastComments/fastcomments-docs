FastComments также поддерживает виджет Page Reacts (также известный как Floating Like button) для Zyro.

Вы можете увидеть его в действии в правом нижнем углу этой страницы!

1. Сначала возьмите код:

[inline-code-attrs-start title = 'Код плавающих лайков Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Затем в Zyro откройте конструктор сайта.
3. Перейдите в «Настройки сайта» в нижнем левом углу.
4. Выберите Интеграции.
5. Добавьте новый код в *конец* поля `Custom code`, и опубликуйте сайт.
6. Вы не увидите виджет в режиме предварительного просмотра, но он появится в опубликованной версии сайта.