Bu örnek, Wix ile uyumlu olması için özel kod kullanır. **Diğer öğreticilerdeki FastComments kod parçacıklarını kullanamayacaksınız.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Adım 3: HTML Diyaloğunu Açın</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Adım 3: HTML Diyaloğunu Açın" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Update:

[inline-code-attrs-start title = 'Wix Yorumları Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

<div class="screenshot white-bg">
    <div class="title">Adım 3: Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Adım 3: Yapıştır ve Kaydet" />
</div>

Şimdi yorum bileşeninin çok küçük bir sürümünü görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">Adım 3: Sonuç</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Adım 3: Sonuç" />
</div>

Sonraki adımda bunu sayfamıza sığacak şekilde konumlandıracak ve boyutlandıracağız.

---