Artık şablon düzenleyicideyiz, yorumları veya canlı sohbeti nerede görüntülemek istediğimize karar vermeliyiz.

Bu örnekte doğrudan videonun altına ekleyeceğiz. Widget'ı sonuna eklemek istediğimiz öğenin üzerine gelin ve `ADD ELEMENT` düğmesine tıklayın:

<div class="screenshot white-bg">
    <div class="title">Öğe Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Öğe Ekle" />
</div>

`CUSTOM JS/HTML` seçeneğini seçin:

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML Seçin</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML Seçin" />
</div>

Şimdi kodumuzu yapıştıracağımız kod düzenleyiciyi açalım.

ClickFunnels bu sonraki adımda biraz kafa karıştırıcıdır.

Yeni öğenin üzerine geldiğinizde *KESİNLİKLE* `Code` seçeneğini seçmemeniz önemlidir. Bunun yerine `SETTINGS` seçeneğini seçin:

<div class="screenshot white-bg">
    <div class="title">SETTINGS Seçin</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS Seçin" />
</div>

Şimdi sağ tarafta `Open Code Editor` düğmesine tıklayabiliriz:

<div class="screenshot white-bg">
    <div class="title">Open Code Editor Düğmesine Tıklayın</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Open Code Editor Düğmesine Tıklayın" />
</div>

Açılan büyük bir kare göreceksiniz. Kodumuzu buraya yapıştırabiliriz. Aşağıdaki kod parçacığını kopyalayın (sağ üstteki kopyala düğmesini kullanın):

[inline-code-attrs-start title = 'ClickFunnels Yayın Sohbeti Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Bu kod parçacığı, videolarla iyi uyum sağlayan Yayın Sohbeti (Streaming Chat) ürünümüze aittir. Bunun yerine, normal sayfalar veya blog yazılarıyla en iyi uyum sağlayan Canlı Yorumlama (Live Commenting) widget kod parçacığını istiyorsanız, bu eğitimin sonunda bulunmaktadır.

Kod parçacığını pencereye yapıştırdığımızda şu şekilde görünmelidir:

<div class="screenshot white-bg">
    <div class="title">Kodu Yapıştır</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Kodu Yapıştır" />
</div>

Şimdi sadece kutuyu kapatmamız gerekiyor:

<div class="screenshot white-bg">
    <div class="title">Kapat</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Kapat" />
</div>

Artık değişikliklerinizi önizleyebilirsiniz! Widget'ı dilediğiniz gibi taşıyın ve nerede en çok hoşunuza gittiğine bakın.

<div class="screenshot white-bg">
    <div class="title">Önizleme</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Önizleme" />
</div>

Başarı! Mobil testi yapmayı unutmayın!

<div class="screenshot white-bg">
    <div class="title">Başarı!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Başarı!" />
</div>