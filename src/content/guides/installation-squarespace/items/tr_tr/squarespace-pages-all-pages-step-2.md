Şimdi aşağıdaki kod parçasını kopyalayabiliriz. Kod parçasının sağ üst köşesinde beliren kopyala düğmesini kullanın.

Kodda yapılandırabileceğiniz birkaç şey var; 4 ila 7. satırlara bakın.

[inline-code-attrs-start title = 'Squarespace Tüm Sayfalar Yorum Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // hesap kimliğiniz

        function tryLoad() {
            // farklı düzenler için yüklemeyi dene
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...sonra kod alanına yapıştırın ve kaydet'e tıklayın. Kodun `FOOTER` bloğunda olduğu şu şekilde görünmelidir:

<div class="screenshot white-bg">
    <div class="title">Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Yapıştır ve Kaydet" />
</div>

Sorun yaşıyorsanız, aşağıya yakın bölümde `"tenantId": "demo"` yazmadığından emin olun. Giriş yaptıysanız, tenant id'inizi göstermelidir. Eğer göstermiyorsa, destek ile iletişime geçin.