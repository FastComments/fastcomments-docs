---
FastComments を BigCommerce のページビルダー経由で追加することは推奨されません。そうすると、コードを希望する各ページに手動で追加する必要があるためです。

ただし、これを行いたい場合は、次のコードスニペットを使用する必要があります。BigCommerce の特性上、他のチュートリアルのコードスニペットは機能しません:

[inline-code-attrs-start title = 'BigCommerce ページビルダー用スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

---