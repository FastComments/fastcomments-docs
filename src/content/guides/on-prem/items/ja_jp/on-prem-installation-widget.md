フロントエンドのコードスニペットとライブラリは、オンプレミス向けでも SaaS 製品と同じです。ただし、`apiHost` と正しいスクリプトパスを指定する必要があります：

[inline-code-attrs-start title = 'オンプレミス用コメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... SSO ペイロードなども渡せます。
    }];
</script>
[inline-code-end]

上は非常に簡単な例です。公式の React、Angular、Vue、Svelte などのライブラリも使用できます。

---