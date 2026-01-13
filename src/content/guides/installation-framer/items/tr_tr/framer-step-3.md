Aşağıda Framer için FastComments Canlı Yorumlar kod parçacığı bulunmaktadır.

[inline-code-attrs-start title = 'FastComments Framer Özel Yorum Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bazı sağlayıcılar kod parçasını async yapıyor
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

Veya alternatif olarak Streaming Chat widget'ını kullanabilirsiniz. Framer Streaming Chat için FastComments kod parçacığı şudur:

[inline-code-attrs-start title = 'FastComments Framer Özel Canlı Sohbet Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // bazı sağlayıcılar kod parçasını async yapıyor
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments, Framer editörünü destekler; bu yüzden kodu yapıştırdıktan sonra şöyle bir şey görmelisiniz (muhtemelen `Publish`'e tıklamanız gerekir):

<div class="screenshot white-bg">
    <div class="title">Yorum Widget Önizlemesi</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Yorum Widget Önizlemesi" />
</div>

Artık sitenizi görüntülediğinizde yorum alanını görmelisiniz! İsterseniz Framer'ın kenar çubuğunda widget'ı tam genişlik olarak ayarlayabilirsiniz.

Framer'ın widget'ların yüksekliğini sınırladığını ve otomatik yeniden boyutlandırmayı desteklemediğini unutmayın; bu yüzden burada sabit yüksekliğe sahip olduğundan Live Chat widget'ını seçtik.