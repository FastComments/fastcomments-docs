Bir sonraki adımda aşağıdaki önceden hazırlanmış widget kodunu kopyalamanız gerekiyor.

FastComments.com'a giriş yapmış olduğunuz sürece aşağıdaki kod parçacığı zaten hesap bilgilerinizi içerecektir. Hadi kopyalayalım:

[inline-code-attrs-start title = 'Super.so FastComments İşbirlikçi Sohbet Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Mevcut örneği temizle
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Mevcut üst çubuğu varsa temizle
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Yeni üst çubuk oluştur
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // FastComments Collab Chat'i başlat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Geçerli yol adını güncelle
            currentPathname = window.location.pathname;
        }

        // Başlangıç yüklemesi
        load();

        // Değişiklikleri her 500ms'de kontrol et
        setInterval(() => {
            // Yol adı değiştiyse yeniden yükle
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Widget kaldırıldıysa yeniden yükle
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Konteyner boşaltıldıysa yeniden yükle
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">Yapıştırılan Kod</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Yapıştırılan Kod" />
</div>

If you see a "this is a demo message" after pasting the code:

- Ensure you're logged into your fastcomments.com account.
- Ensure you have 3rd party cookies enabled.
- Then refresh this page and copy the code snippet again. It should have `tenantId` populated with your tenant's identifier.