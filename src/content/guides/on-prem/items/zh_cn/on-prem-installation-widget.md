用于 On-Prem 的前端代码片段和库与 SaaS 产品相同。但是，您必须指定 `apiHost` 和正确的脚本路径：

[inline-code-attrs-start title = '本地部署的评论代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... 也可以传递 SSO 有效载荷等。
    }];
</script>
[inline-code-end]

以上是一个非常简单的示例。我们也可以使用第一方的 React、Angular、Vue、Svelte 等库。