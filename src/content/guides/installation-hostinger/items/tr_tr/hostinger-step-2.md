Şimdi widget kodumuzu ekleyelim.

Aşağıdaki kodu kopyalayın. [fastcomments.com](https://fastcomments.com) hesabınıza giriş yaptığınızdan emin olun ve hesap bilgilerinizle kodun önceden doldurulması için sayfayı yeniden yükleyin; aksi takdirde demo kodu gösterilecektir.

Şimdi kodu kopyalayalım:

[inline-code-attrs-start title = 'Hostinger Yorum Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Şimdi site oluşturucumuza geri dönüp `Enter code`'a tıklayın:

<div class="screenshot white-bg">
    <div class="title">Kodu girin</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Kodu girin" />
</div>

### Not!

Yukarıdaki kodu kullanmanız ve diğer dokümantasyonlardaki kod parçacıklarını kullanmamanız önemlidir; bu snippet özel olarak Hostinger için hazırlanmıştır.

Şu anda aşağıdaki gibi, boş görünen bir alanınız olmalı. Bu beklenmektedir. Widget'ın olması gereken alanın üzerine fare ile gelin:

<div class="screenshot white-bg">
    <div class="title">Kod Bileşeni Eklendi</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Kod Bileşeni Eklendi" />
</div>

Şimdi widget'ı istediğiniz boyuta sürükleyin; görünmeye başladığını göreceksiniz:

<div class="screenshot white-bg">
    <div class="title">Boyutlandırın</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Boyutlandırın" />
</div>

...ve şimdi önizleyin ve kaydedin!