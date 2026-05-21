If you can't install the [Shopify App Store app](https://apps.shopify.com/fastcomments), you can still add FastComments by editing your theme. This path is useful when you want to wire up a FastComments tenant you already own, or when you're embedding on a Shopify storefront where the app isn't an option.

The app-based install is the supported path for most stores. Reach for this only if the app doesn't fit.

### Adım 1: Shopify'ın yerleşik yorumlarını devre dışı bırakın

Shopify yönetiminizde, **Blog posts > Manage blogs** bölümüne gidin, her blogu açın ve sağ panelde **Comments are disabled** seçeneğini ayarlayın. Kaydedin.

Bu, Shopify'ın yerleşik yorum sisteminin FastComments ile birlikte gösterilmesini durdurur.

### Adım 2: Blog tema şablonunu açın

Shopify yönetiminizde:

1. Go to **Online Store > Themes**.
2. Under your current theme, click **Actions > Edit code**.
3. In the file browser on the left, open **Sections** and click `main-article.liquid`.

Bu, Shopify'ın tek bir blog makalesi için render ettiği şablondur.

### Adım 3: FastComments kod parçacığını yapıştırın

`main-article.liquid` dosyasının yaklaşık 100. satırına, makale gövdesinin kapanış `</div>` etiketinin hemen sonrasına gidin. Aşağıdaki kod parçacığını yapıştırın:

[inline-code-attrs-start title = 'Shopify FastComments Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Replace `"demo"` with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Click **Save**.

### Adım 4: Mağaza alan adınızı yetkilendirin

Canlı mağazanızda bir blog gönderisini açın. Eğer yorum bileşeni yerine bir yetkilendirme hatası görürseniz, FastComments'in mağazanızın bu tenant'ı kullanmasına izin verildiğini bilmesi gerekir. Bakınız [Alan Adı Hataları](/guide-installation-shopify.html#shopify-domain-errors).

### FastComments'i diğer sayfalara ekleme

Aynı kod parçacığı, ürün sayfaları, özel sayfalar ve ana sayfa dahil olmak üzere herhangi bir Liquid şablonunda çalışır. Yorumların görünmesini istediğiniz yere yapıştırın ve sayfa başına sabit bir tanımlayıcı istiyorsanız `urlId` değerini ayarlayın (örneğin, bir ürün şablonunda `urlId: "{{ product.id }}"`).