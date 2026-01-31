---
On-Prem 的前端程式碼範例和函式庫與 SaaS 產品相同。然而，您必須指定 `apiHost` 與正確的腳本路徑：

[inline-code-attrs-start title = 'On-Prem 的評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... 也可以傳遞 SSO payload 等等。
    }];
</script>
[inline-code-end]

以上是一個非常簡單的範例。我們也可以使用第一方的 React、Angular、Vue、Svelte 等函式庫。

---