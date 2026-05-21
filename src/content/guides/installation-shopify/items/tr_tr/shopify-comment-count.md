The **FastComments - Comment Count** bloğu tek bir sayfa için küçük bir yorum sayısı görüntüler. Bunu blog gönderi listelerinde, ürün kartlarında veya yorumların bulunduğu bir sayfaya bağlantı veren herhangi bir şablonda kullanın; böylece ziyaretçiler tıklamadan önce her konunun ne kadar aktif olduğunu görebilir.

### Bloğu ekleyin

1. Shopify tema düzenleyicisini açın.
2. Sayımın görünmesini istediğiniz şablonu açın. Örneğin, **Blog** şablonu (gönderi listesi) veya bir ürün listeleme bölümü.
3. Her öğeyi render eden bölümde **Add block** (Bloğu ekle) öğesine tıklayın.
4. **Apps** altında **FastComments - Comment Count** öğesini seçin.
5. **Save** (Kaydet) öğesine tıklayın.

### Ayarlar

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Hangi FastComments kiracısından (tenant) sayımın okunacağını geçersiz kılar. Mağazanın otomatik yapılandırılmış kiracısını kullanmak için boş bırakın. | (boş) |
| Custom URL ID | Sayımın aradığı sayfa tanımlayıcısını geçersiz kılar. Sayım, takip ettiği FastComments bloğundan farklı bir sayfadaysa bunu kullanın. | (otomatik algılandı) |

### Sayımın yorum dizisiyle eşleşmesi

Comment Count bloğu, **FastComments** bloğu ile aynı otomatik algılama mantığını kullanır:

- Blog gönderi şablonu: `shopify-article-{article.id}`
- Ürün şablonu: `shopify-product-{product.id}`
- Diğer şablonlar: istek yolu

Bir sayfada **FastComments** bloğunda bir **Custom URL ID** ayarlarsanız, aynı konuyu işaret etmeleri için Comment Count bloğunda da aynı Custom URL ID'yi ayarlayın.

### İpuçları

- Sayfadaki her öğe için sayımlar tek bir istekte alınır, bu nedenle uzun bir listede bloğu her öğeye eklemenin ekstra bir tur maliyeti yoktur.
- Bir listelemedeki her makale veya ürün için bir Comment Count bloğu beklenen kullanımdır; bloğu ihtiyaç duyduğunuz kadar ekleyebilirsiniz.