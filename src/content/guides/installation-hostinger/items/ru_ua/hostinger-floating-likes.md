---
FastComments также поддерживает виджет Page Reacts (также известный как плавающая кнопка «Нравится») для Hostinger.

Вы можете увидеть его в действии в правом нижнем углу этой страницы!

### Примечание!

Эти инструкции предназначены для конструктора сайтов Hostinger. Если вы используете Hostinger *WordPress*, просто возьмите код ниже и добавьте его на ваш сайт WordPress с помощью [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), это бесплатный и простой плагин для добавления небольших фрагментов кода на сайт.

1. Сначала скопируйте код:

[inline-code-attrs-start title = 'Код плавающих лайков Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Затем в Hostinger откройте конструктор сайта.
3. Перейдите в «Настройки сайта» в левом нижнем углу.
4. Выберите «Интеграции».
5. Добавьте новый код в *конец* поля `Custom code`, и опубликуйте сайт.
6. В режиме предварительного просмотра виджет не будет виден, но он появится в опубликованной версии сайта.

---