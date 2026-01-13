Şimdi widget kodumuzu ekleyelim.

Aşağıdaki kodu kopyalayın. [fastcomments.com](https://fastcomments.com) sitesine giriş yapmış olduğunuzdan emin olun ve sayfa önceden hesap bilgilerinizle doldurulsun diye girişli değilseniz bu sayfayı yeniden yükleyin, aksi takdirde demo kodu gösterilecektir.

Şimdi kodu kopyalayalım:

[inline-code-attrs-start title = 'Zyro Yorum Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Şimdi site oluşturucumuza geri dönelim ve `Enter code` öğesine tıklayın:

<div class="screenshot white-bg">
    <div class="title">Kodu Gir</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Kodu Gir" />
</div>

### Not!

Yukarıdaki kodu kullanmanız ve diğer belgelerdeki kod parçacıklarını kullanmamanız önemlidir; çünkü bu kod parçası Zyro için özel olarak hazırlanmıştır.

Şu anda aşağıdakine benzer, boş görünen bir alanınız olmalı. Bu beklenen bir durumdur. Widget'ın olması gereken alanın üzerine farenizi getirin:

<div class="screenshot white-bg">
    <div class="title">Kod Widget Eklendi</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Kod Widget Eklendi" />
</div>

Şimdi widget'ı istediğiniz boyuta sürükleyin; görünmesini göreceksiniz:

<div class="screenshot white-bg">
    <div class="title">Yeniden Boyutlandır</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Yeniden Boyutlandır" />
</div>

...ve şimdi önizleyin ve kaydedin!