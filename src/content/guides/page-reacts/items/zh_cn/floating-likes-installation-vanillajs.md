安装很简单：

[inline-code-attrs-start title = '浮动点赞 代码示例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

构造函数的类型签名为：

[inline-code-attrs-start title = '配置'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - 更改此项以设置页面/文章 id。默认情况下，它是页面的 URL。
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

它支持 SSO，可将点赞与用户账户绑定以进行身份验证。

目前仅支持 VanillaJS。如果您希望将此小部件加入我们的某个客户端库，请告诉我们！ 

### 异步版本

[inline-code-attrs-start title = '浮动点赞 异步代码示例'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

我们也可以传递 Secure SSO 或 Simple SSO 的负载：

[inline-code-attrs-start title = '浮动点赞 安全 SSO 代码示例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // 传递 sso 对象
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = '浮动点赞 简单 SSO 代码示例'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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