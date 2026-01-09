---
On-Premのフロントエンド用コードスニペットとライブラリはSaaS製品と同じです。ただし、`apiHost` と正しいスクリプトパスを指定する必要があります：

[inline-code-attrs-start title = 'オンプレ用のコメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... SSO のペイロードなども渡すことができます。
    });
</script>
[inline-code-end]

上記は非常に単純な例です。1stパーティの React、Angular、Vue、Svelte などのライブラリを使用することもできます。

---