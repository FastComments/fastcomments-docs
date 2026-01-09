安裝很簡單：

[inline-code-attrs-start title = '浮動按讚 程式碼範例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

建構函式的型別簽名如下：

[inline-code-attrs-start title = '設定'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - 更改此項以設定頁面/文章的 ID。預設為該頁面的 URL。
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

它支援 SSO，可將反應綁定到使用者帳戶以進行驗證。

目前僅支援 VanillaJS。如果您希望將此元件新增至我們的某個用戶端函式庫，請告知我們！

### 非同步版本

[inline-code-attrs-start title = '浮動按讚 非同步 程式碼範例'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

### 使用 SSO

我們也可以傳遞 Secure SSO 或 Simple SSO 的有效載荷：

[inline-code-attrs-start title = '浮動按讚 安全 SSO 程式碼範例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // 傳入 sso 物件
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = '浮動按讚 簡易 SSO 程式碼範例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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