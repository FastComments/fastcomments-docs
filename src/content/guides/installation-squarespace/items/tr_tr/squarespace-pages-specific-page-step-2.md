Şimdi aşağıdaki kod parçasını kopyalayabiliriz. Kod parçasının sağ üst köşesinde görünen kopyala düğmesini kullanın.

Kodda yapılandırabileceğiniz birkaç şey var, 4 ila 7. satırlara bakın.

[inline-code-attrs-start title = 'Squarespace Tek Sayfa Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // hesap kimliğiniz
    }];
</script>
[inline-code-end]

Aşağıdaki gibi görünmelidir:

<div class="screenshot white-bg">
    <div class="title">Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Yapıştır ve Kaydet" />
</div>

Şimdi sağ üstte Kaydet'e tıklayın.

`Preview in Safe Mode` seçeneğinin çalışmayacağını unutmayın, ancak widget sitenizi ziyaret ettiğinizde görünecektir.

Sorun yaşıyorsanız, aşağıya yakın kısımda `"tenantId": "demo"` yazmadığından emin olun. Oturum açmışsanız orada tenant id'niz görünmelidir. Değilse, destek ile iletişime geçin.