Şimdi FastComments widget kodunu sitenize ekleyeceğiz. Bu kod, başlığı `FastComments Goes Here` olan tüm formları arayacak ve
onları FastComments ile değiştirecektir.

Site düzenleyicisinin sol altındaki `Settings` bölümüne gidelim:

<div class="screenshot white-bg">
    <div class="title">Ayarları Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Ayarları Aç" />
</div>

`Custom Head Code` bölümünü açın:

<div class="screenshot white-bg">
    <div class="title">Custom Head Code'u Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Custom Head Code'u Aç" />
</div>

Ionos için FastComments widget kodunun **özel bir sürümüne** ihtiyacımız var. **Diğer eğitimlerden alınan kod parçacıkları çalışmayacaktır.**

Şimdi aşağıdaki kodu kopyalayın:

[inline-code-attrs-start title = 'Ionos FastComments Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // we get the element that is not full width
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...ve gösterildiği gibi yapıştırın:

<div class="screenshot white-bg">
    <div class="title">Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Yapıştır ve Kaydet" />
</div>