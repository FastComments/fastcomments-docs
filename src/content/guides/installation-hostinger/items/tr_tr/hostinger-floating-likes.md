---
FastComments ayrıca Hostinger için Sayfa Beğenileri (diğer adıyla Yüzen Beğeni düğmesi) bileşenini destekler.

Bunu bu sayfanın sağ alt köşesinde çalışırken görebilirsiniz!

### Not!

Bu talimatlar Hostinger Site Builder içindir. Eğer Hostinger *WordPress* kullanıyorsanız, aşağıdaki kodu kopyalayıp WordPress sitenize ekleyin
[WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) kullanarak; bu, sitenize küçük kod parçacıkları eklemek için ücretsiz ve kolay bir eklentidir.

1. İlk olarak, kodu alın:

[inline-code-attrs-start title = 'Hostinger Yüzen Beğeni Kodu'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Daha sonra, Hostinger'da Site Builder'ı açın.
3. Sol alt köşedeki Site Ayarları'na gidin.
4. Entegrasyonlar'ı seçin.
5. Yeni kodu `Custom code` alanının *sonuna* ekleyin ve sitenizi yayınlayın.
6. Önizleme modunda bileşeni görmeyeceksiniz, ancak sitenin yayınlanmış sürümünde görünecektir.

---