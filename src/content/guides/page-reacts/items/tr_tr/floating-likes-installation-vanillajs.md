Kurulum basittir:

[inline-code-attrs-start title = 'Yüzen Beğeniler Kod Örneği'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Yapıcının (constructor) tür imzası:

[inline-code-attrs-start title = 'Yapılandırma'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - Sayfa/makale kimliğini ayarlamak için bunu değiştirin. Varsayılan olarak sayfa URL'sidir.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

Tepkileri kullanıcının hesabına kimlik doğrulama için bağlamak üzere sso'yu destekler.

Şu anda yalnızca VanillaJS desteklenmektedir. Bu widget'ın istemci kütüphanelerimizden birine eklenmesini isterseniz, bize bildirin! 

### Asenkron Sürüm

[inline-code-attrs-start title = 'Yüzen Beğeniler Asenkron Kod Örneği'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

### SSO ile

Ayrıca Secure SSO veya Simple SSO yükleri de iletebiliriz:

[inline-code-attrs-start title = 'Yüzen Beğeniler Güvenli SSO Kod Örneği'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // sso nesnesini gönderin
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Yüzen Beğeniler Basit SSO Kod Örneği'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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