Bu örnek, Wix ile uyumlu olmak için özel kod kullanır. **Diğer eğitimlerden aldığınız FastComments kod parçacıklarını kullanamayacaksınız.**

Özel HTML iletişim kutumuzu eklemek için formu açın; bunun için `Enter Code` seçeneğine tıklayıp `HTML` seçeneğini seçin:

<div class="screenshot white-bg">
    <div class="title">Adım 3: HTML İletişim Kutusunu Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Adım 3: HTML İletişim Kutusunu Aç" />
</div>

Aşağıdaki HTML parçacığını kopyalayın, iletişim kutusuna yapıştırın ve Update'e tıklayın:

[inline-code-attrs-start title = 'Wix Yorumlar Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

Artık yorum bileşeninin çok küçük bir versiyonunu görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">Adım 3: Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Adım 3: Yapıştır ve Kaydet" />
</div>

<div class="screenshot white-bg">
    <div class="title">Adım 3: Sonuç</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Adım 3: Sonuç" />
</div>

Sırada, bunu sayfamıza uygun şekilde konumlandırıp boyutlandıracağız.