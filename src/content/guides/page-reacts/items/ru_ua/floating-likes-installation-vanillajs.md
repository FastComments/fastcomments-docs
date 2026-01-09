Установка проста:

[inline-code-attrs-start title = 'Пример кода Floating Likes'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Сигнатура типа конструктора:

[inline-code-attrs-start title = 'Конфигурация'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - Измените это, чтобы задать идентификатор страницы/статьи. По умолчанию это URL страницы.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

Он поддерживает sso для привязки реакций к учётной записи пользователя для аутентификации.

В настоящее время поддерживается только VanillaJS. Если вы хотите, чтобы этот виджет был добавлен в одну из наших клиентских библиотек, дайте нам знать! 

### Асинхронная версия

[inline-code-attrs-start title = 'Пример асинхронного кода Floating Likes'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: 'demo'
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

### С SSO

Также можно передать данные Secure SSO или Simple SSO:

[inline-code-attrs-start title = 'Пример кода Secure SSO для Floating Likes'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // передать объект sso
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Пример кода Simple SSO для Floating Likes'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        simpleSSO: {
            id: 'some-user-id',
            username: 'some username',
            email: 'some@email.com',
        }
    });
</script>
[inline-code-end]

---