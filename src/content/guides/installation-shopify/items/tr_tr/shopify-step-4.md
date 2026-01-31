Şimdi `100` numaralı satıra aşağı kaydıracağız:

<div class="screenshot white-bg">
    <div class="title">100. Satıra Kaydır</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="100. Satıra Kaydır" />
</div>

Şimdi aşağıdaki kod parçasını kopyalayın; bu kod **özellikle Shopify için tasarlanmıştır - diğer eğitimlerden alınan kod parçalarını kullanmayın**:

[inline-code-attrs-start title = 'Shopify FastComments Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Şimdi imleci `line 101` - `</div>`'in hemen sonrasına - koyup yapıştırmak istiyoruz. Şunun gibi bir şeyiniz olmalı:

<div class="screenshot white-bg">
    <div class="title">FastComments Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments Kodunu Ekle" />
</div>

Şimdi kaydedebiliriz:

<div class="screenshot white-bg">
    <div class="title">Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Kaydet" />
</div>

---