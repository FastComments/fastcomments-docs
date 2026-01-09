## Super.so Notion Makalelerinize Canlı Yorum Widget Eklemek

Collab Chat'e ek olarak, Notion makalelerinizin altına geleneksel bir yorum widget'ı ekleyebilirsiniz. Bu, okuyucuların yorum bırakmasına ve tüm makale hakkında tartışma yapmasına olanak tanır.

### Kurulum Adımları

Aşağıdaki kodu kopyalayın ve Super.so site ayarlarınızın `Body` bölümüne yapıştırın:

[inline-code-attrs-start title = 'Super.so FastComments Canlı Yorum Widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Mevcut örneği temizle
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Yeni hedef oluştur
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments'i başlat
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Mevcut pathname'i güncelle
            currentPathname = window.location.pathname;
        }

        // İlk yükleme
        load();

        // Değişiklikler için her 500ms'de kontrol et
        setInterval(() => {
            // Pathname değiştiyse yeniden yükle
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
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Önemli Notlar

- Yorum widget'ı Notion makalelerinizin altında görünecektir
- Her sayfa, URL yoluna göre kendi benzersiz yorum dizisine sahip olur
- FastComments hesabınızdaki gerçek tenant ID'niz ile `demo`'yu değiştirdiğinizden emin olun
- Widget, Super.so'nun dinamik sayfa yüklemelerini otomatik olarak yönetir

---