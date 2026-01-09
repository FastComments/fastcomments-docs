Die Installation ist einfach:

[inline-code-attrs-start title = 'Floating Likes Code-Beispiel'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Die Typsignatur des Konstruktors lautet:

[inline-code-attrs-start title = 'Konfiguration'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - Ändern Sie dies, um die Seiten-/Artikel-ID festzulegen. Standardmäßig ist es die Seiten-URL.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

Es unterstützt sso, um die Reaktionen mit dem Benutzerkonto zur Authentifizierung zu verknüpfen.

Derzeit wird nur VanillaJS unterstützt. Wenn Sie möchten, dass dieses Widget zu einer unserer Client-Bibliotheken hinzugefügt wird, lassen Sie es uns wissen! 

### Asynchrone Version

[inline-code-attrs-start title = 'Floating Likes Code Async-Beispiel'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

### Mit SSO

Wir können außerdem Secure SSO- oder Simple SSO-Payloads übergeben:

[inline-code-attrs-start title = 'Floating Likes Secure SSO Code-Beispiel'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // sso-Objekt übergeben
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Floating Likes Simple SSO Code-Beispiel'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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