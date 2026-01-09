Şimdi aşağıdaki kod parçacığını kopyalayabiliriz. Parçacığın sağ üst köşesinde görünen kopyala düğmesini kullanın.

Koddaki birkaç şeyi yapılandırabilirsiniz, 4 ile 7. satırlara bakın.

[inline-code-attrs-start title = 'Squarespace Tek Sayfa Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // hesap kimliğiniz

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Şunun gibi görünmelidir:

<div class="screenshot white-bg">
    <div class="title">Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Yapıştır ve Kaydet" />
</div>

Şimdi sağ üstte Kaydet'e tıklayın.

`Preview in Safe Mode` seçeneğinin çalışmayacağını unutmayın, ancak widget sitenizi ziyaret ettiğinizde görünecektir.

Sorun yaşıyorsanız, sayfanın alt kısmına yakın yerde `"tenantId": "demo"` yazmadığından emin olun. Giriş yaptıysanız, tenant kimliğinizi göstermelidir. Değilse, destek ile iletişime geçin.