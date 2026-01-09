Custom Code sekmesinin **Footer** bölümüne aşağıdaki kodu yapıştırın:

[inline-code-attrs-start title = 'Typeflo.io Canlı Yorum Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Kodu Footer Bölümüne Yapıştırın</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="FastComments Kodunu Footer Bölümüne Yapıştırın" />
</div>

Kodu yapıştırdıktan sonra değişiklikleri uygulamak için **Save** düğmesine tıklayın.

Note: Bu kod, FastComments widget'ını Typeflo.io blog gönderilerinizde en uygun konuma dinamik olarak yerleştirmek için mantık içerir. Diğer kod parçacıkları Typeflo.io'nun düzeniyle düzgün çalışmayabilir.

Kaydolduktan sonra `'demo'` değerini gerçek FastComments tenant kimliğinizle değiştirmeyi unutmayın. FastComments.com'a giriş yaptıysanız, bu değer zaten değiştirilmiş olmalıdır.