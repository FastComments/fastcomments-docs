---
On-Prem 的前端程式碼範例與函式庫與 SaaS 產品相同。然而，您必須指定 `apiHost` 並使用正確的腳本路徑：

[inline-code-attrs-start title = '內部部署評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... 也可以傳遞 SSO 載荷等。
    });
</script>
[inline-code-end]

上述是一個非常簡單的範例。我們也可以使用官方第一方的 React、Angular、Vue、Svelte 等函式庫。

---