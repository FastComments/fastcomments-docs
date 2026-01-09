Şimdi widget kodumuzu ekleyelim.

Aşağıdaki kodu kopyalayın. Hesap bilgilerinizle kodun önceden doldurulmuş olması için [fastcomments.com](https://fastcomments.com) sitesine giriş yaptığınızdan emin olun ve yapmadıysanız bu sayfayı yeniden yükleyin; aksi takdirde demo kodu gösterilecektir.

Şimdi kodu kopyalayalım:

[inline-code-attrs-start title = 'Hostinger Yorumları Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Şimdi site oluşturucumuza geri dönelim ve `Enter code` öğesine tıklayalım:

<div class="screenshot white-bg">
    <div class="title">Kodu Gir</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Kodu Gir" />
</div>

### Not!

Yukarıdaki kodu kullanmanız önemlidir; diğer dökümantasyonlardaki kod parçacıklarını kullanmayın, çünkü bu parça özellikle Hostinger için hazırlanmıştır.

Şimdi aşağıdaki gibi boş görünen bir şeyiniz olmalıdır. Bu beklenmektedir. Widget'ın olması gereken alanın üzerine fareyle gelin:

<div class="screenshot white-bg">
    <div class="title">Kod Widget'ı Eklendi</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Kod Widget'ı Eklendi" />
</div>

Şimdi widget'ı istediğiniz boyuta sürükleyin, göründüğünü göreceksiniz:

<div class="screenshot white-bg">
    <div class="title">Boyutunu Değiştir</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Boyutunu Değiştir" />
</div>

...ve şimdi önizleyin ve kaydedin!

---