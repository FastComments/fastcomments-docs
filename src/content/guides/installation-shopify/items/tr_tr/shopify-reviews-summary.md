---
The **FastComments - İncelemeler Özeti** bloğu bir sayfa için toplu yıldız puanını ve inceleme dökümünü gösterir. Standart inceleme düzeni için ürün şablonlarındaki **FastComments** bloğuyla eşleştirin: özet yukarıda, inceleme formu ve incelemeler aşağıda.

### Önkoşul: Derecelendirmeler & İncelemeler'i ayarlama

İncelemeler Özeti bloğu, mağazanız için yapılandırdığınız değerlendirme sorularını gösterir. Önce bunları ayarlayın:

1. Shopify yönetiminizde FastComments uygulamasını açın.
2. **Ratings & Reviews Helper** karosuna tıklayın (veya doğrudan [Derecelendirmeler & İncelemeler Yardımcısı](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) sayfasını açın).
3. Her değerlendiricinin yanıtlamasını istediğiniz soruları ekleyin (genel yıldız puanı, "beden nasıl uydu", vb.).

Sorular yapılandırılmamışsa, özet bloğunun toplayacağı veri olmaz.

### Bloğu ekleyin

1. Shopify tema düzenleyicisini açın.
2. **Ürün** şablonunu açın (veya özetin yer almasını istediğiniz sayfa şablonunu).
3. Sayfa bölümünün üst kısmına yakın, **Bloğu ekle**'ye tıklayın; bu, **FastComments** bloğunun olacağı yerin üzerinde olmalıdır.
4. **Uygulamalar** altında **FastComments - İncelemeler Özeti**'ni seçin.
5. Ziyaretçilerin inceleme bırakabilmesi için aynı sayfada daha aşağıya bir **FastComments** bloğu ekleyin (henüz eklemediyseniz).
6. **Kaydet**'e tıklayın.

### Ayarlar

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Özetin hangi FastComments kiracısından okuma yapacağını geçersiz kılar. Mağazanın otomatik olarak yapılandırdığı kiracıyı kullanmak için boş bırakın. | (boş) |
| Custom URL ID | Özetin toplama yaptığı sayfa kimliğini geçersiz kılar. Özet, yansıttığı FastComments bloğundan farklı bir sayfadaysa bunu kullanın. | (otomatik algılanır) |

### Özetin incelemelerle nasıl eşleştiği

İncelemeler Özeti bloğu, **FastComments** bloğuyla aynı otomatik algılama mantığını kullanır:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Normal bir ürün sayfası için özet ile yorum dizisi otomatik olarak bir URL kimliğini paylaşır; herhangi bir yapılandırma gerekmez.

### İpuçları

- Özet salt okunurdur. İnceleme toplamak için aynı sayfada bir **FastComments** bloğuna ihtiyacınız vardır.
- İncelemeleri topladıktan sonra Derecelendirmeler & İncelemeler Yardımcısı'nda değerlendirme sorularını değiştirirseniz, özet yeni soru kümesine göre yeniden hesaplanır.

---