Az önce eklediğiniz HTML öğesine tıklayın. Görüntülenen özellik düzenleyicisinde, aşağıdaki kodu HTML alanına yapıştırın:

[inline-code-attrs-start title = 'Bubble.io Canlı Yorumlama Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bubble kod parçacığını genellikle async olacak şekilde değiştirir
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">FastComments Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="FastComments Kodunun HTML Öğesine Yerleştirilmesi" />
</div>

Not: Bu kod, FastComments'in Bubble'ın dinamik ortamında düzgün şekilde yüklenmesini sağlamak için bir yeniden deneme mekanizması içerir.
Diğer kod parçacıkları çalışmayacaktır.

Kayıt olduktan sonra `'demo'` değerini gerçek FastComments tenant ID'niz ile değiştirin. FastComments.com'a giriş yaptıysanız, bu zaten değiştirilmiş olacaktır.