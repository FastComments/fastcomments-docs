Flex faturası, her kullanım türü için bir kalem içeren, her kalemin miktarı ve oranını gösteren bir liste halidir; örneğin `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Oranlar [fiyatlandırma sayfasında](https://fastcomments.com/traffic-pricing) ve [Fatura Bilgileri](https://fastcomments.com/auth/my-account/billing-info) sayfanızda yer alır. Bu bölüm her miktarın ne anlama geldiğini açıklar.

### Ay Boyunca Sayılan

Bunlar her ay sıfırlanır ve takvim ayının ilk gününden son gününe kadar (UTC) sayılır:

- **Page Loads**: yorum widget'ının veya akışın sayfalarınızda yüklendiği her sefer. Yorum widget'ının her yüklenmesi, kaç yorum gösterdiğine bakılmaksızın sayılır. Yorum dizisi başına veya sayfa başına ücret alınmaz.
- **API Credits**: API anahtarınızla REST API'ye yapılan çağrılar.
- **Small Widgets Credits**: yorum sayıları ve rozetler gibi küçük widget'ların yüklenmeleri.
- **LLM Tokens**: [AI Agents](/guide-ai-agents.html) ve diğer LLM özellikleri tarafından kullanılan token'lar.

### Ay Sonunda Sayılan

Bunlar, fatura oluşturulduğunda alınan bir anlık görüntüdür:

- **Comments Stored**: hesabınızdaki toplam yorum sayısı, yalnızca bu ay eklenenler değil. Yorum silmek bu sayıyı azaltır; ancak cevapları olan silinmiş bir yorum, cevapların hâlâ anlamlı olabilmesi için yer tutucu olarak tutulur.
- **SSO Users**: sitenizin oluşturduğu toplam SSO kullanıcı sayısı, uzun zaman önce bir kez yorum yapmış kişiler dahil. Bu, faturanızın en büyük kalemiyse, yalnızca ay içinde giriş yapan kullanıcıların sayıldığı aylık aktif kullanıcı bazlı faturalandırma için desteğe danışın.
- **Moderators**, **Admins**, **SSO Moderators** ve **SSO Admins**: koltuklar, [Admin ve Moderator Koltukları](#admin-and-moderator-seats) bölümünde ele alınmıştır. İlk admin ücretsizdir.
- **Domains**: hesabınızdaki siteler. İlk site ücretsizdir ve her ek site, hesapta kaldığı sürece aylık olarak faturalandırılır. Bir alt alan adı (subdomain) yalnızca kendi başına ayrı bir site olarak eklerseniz sayılır; tek bir üst düzey alan adı alt alan adlarını kapsar. [Multiple Sites](/guide-multiple-sites.html#domain-limits) bölümüne bakın.
- **Managed Tenants**: bu hesap tarafından yönetilen alt hesaplar, varsa, ayrıca bunların kendi kullanımları ayrı satırlar olarak.

### Minimum

Satırlar toplamı Flex minimumunun altındaysa, fatura minimuma tamamlanır. Bir admin ve bir alan adına sahip küçük bir site yalnızca minimumu öder.

### Faturayı Önizleme

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) sayfası, bir sonraki faturanın her satırını bugünün rakamlarıyla gösterir; böylece sürpriz olmaz. Sayfa yüklemesi, API ve küçük widget sayıları bir dakikaya kadar geride kalabilir; geri kalanı canlıdır.

### Diğer Para Birimleri

Flex kullanımı ABD doları cinsinden fiyatlandırılır. Fatura para biriminiz farklıysa, her satır faturanın oluşturulduğu günkü döviz kuruyla dönüştürülür; böylece aynı kullanım aydan aya biraz farklı tutarlara mal olabilir. [Currencies](#currencies) bölümüne bakın.