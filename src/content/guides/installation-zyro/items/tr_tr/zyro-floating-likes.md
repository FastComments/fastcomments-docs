FastComments ayrıca Zyro için Page Reacts (diğer adıyla Floating Like düğmesi) widget'ını destekler.

Bunu bu sayfanın sağ alt köşesinde çalışırken görebilirsiniz!

1. İlk olarak, kodu alın:

[inline-code-attrs-start title = 'Zyro Yüzen Beğeniler Kodu'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Sonra, Zyro'da site oluşturucusunu açın.
3. Sol alt köşedeki Site Ayarları'na gidin.
4. Entegrasyonlar'ı seçin.
5. Yeni kodu `Custom code` alanının *sonuna* ekleyin ve sitenizi yayınlayın.
6. Önizleme modunda widget'ı görmeyeceksiniz, ancak sitenin yayınlanmış versiyonunda görünecektir.