[Shopify App Store uygulamasını](https://apps.shopify.com/fastcomments) yükleyemiyorsanız, temanızı düzenleyerek FastComments'i yine de ekleyebilirsiniz. Bu yol, zaten sahip olduğunuz bir FastComments tenant'ını bağlamak istediğinizde veya uygulamanın bir seçenek olmadığı bir Shopify mağazasına gömme yaptığınızda kullanışlıdır.

Uygulama tabanlı kurulum, çoğu mağaza için desteklenen yoldur. Buna yalnızca uygulama uygun değilse başvurun.

### Adım 1: Shopify'ın yerel yorumlarını devre dışı bırakın

Shopify yönetim panelinizde **Blog posts > Manage blogs** bölümüne gidin, her blogu açın ve sağ panelde **Comments are disabled** seçeneğini ayarlayın. Kaydedin.

Bu, Shopify'ın yerleşik yorumlama özelliğinin FastComments ile birlikte gösterilmesini engeller.

### Adım 2: Blog tema şablonunu açın

Shopify yönetim panelinizde:

1. **Online Store > Themes** bölümüne gidin.
2. Mevcut temanızın altında **Actions > Edit code** seçeneğine tıklayın.
3. Soldaki dosya tarayıcısında **Sections** bölümünü açın ve `main-article.liquid` dosyasına tıklayın.

Bu, Shopify'ın tek bir blog yazısı için oluşturduğu şablondur.

### Adım 3: FastComments kod parçacığını yapıştırın

`main-article.liquid` dosyasında yaklaşık 100. satıra, makale gövdesinin kapanış `</div>` etiketinin hemen sonrasına gidin. Aşağıdaki kod parçacığını yapıştırın:

[inline-code-attrs-start title = 'Shopify FastComments Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

`"demo"` değerini, [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret) adresinden alacağınız kendi Tenant ID'niz ile değiştirin. **Save** seçeneğine tıklayın.

### Adım 4: Mağaza alan adınızı yetkilendirin

Canlı mağazanızda bir blog yazısı açın. Yorum widget'ı yerine bir yetkilendirme hatası görüyorsanız, FastComments'in mağazanızın bu tenant'ı kullanmaya yetkili olduğunu bilmesi gerekir. Bkz. [Alan Adı Hataları](/guide-installation-shopify.html#shopify-domain-errors).

### FastComments'i diğer sayfalara ekleme

Aynı kod parçacığı, ürün sayfaları, özel sayfalar ve ana sayfa dahil olmak üzere herhangi bir Liquid şablonunda çalışır. Yorumların görünmesini istediğiniz yere yapıştırın ve sayfa başına sabit bir tanımlayıcı istiyorsanız `urlId` değerini ayarlayın (örneğin, bir ürün şablonunda `urlId: "{{ product.id }}"`).