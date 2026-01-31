---
On-Prem için ön uç kod parçacıkları ve kütüphaneler SaaS ürünü ile aynıdır. Ancak `apiHost` ve doğru betik yolunu belirtmelisiniz:

[inline-code-attrs-start title = 'On-Prem İçin Yorum Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... SSO yükü vb. de gönderilebilir.
    }];
</script>
[inline-code-end]

Yukarıdaki çok basit bir örnektir. Ayrıca birinci taraf React, Angular, Vue, Svelte vb. kütüphaneleri de kullanabiliriz.

---