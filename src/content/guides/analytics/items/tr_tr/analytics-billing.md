Faturalandırma veya hız sınırına tabi olabileceğiniz tüm metrikler [Faturalandırma Analitikleri Sayfası](https://fastcomments.com/auth/my-account/analytics/billing) üzerinde raporlanır.

Takip edilen örnek metriklerden bazıları:

- Sayfa Yüklemeleri (geçerli ay için)
- API Kredileri (geçerli ay için)
- Depolanan Yeni Yorumlar (geçerli ay için)
- Tenant Kullanıcıları
- SSO Kullanıcıları (yönetici/moderatör izinleri olmayan normal SSO kullanıcıları)
- SSO Yöneticileri (`isAccountOwner` veya `isAdminAdmin` yönetici bayraklarına sahip SSO kullanıcıları - normal SSO kullanıcılarından ayrı olarak faturalandırılır)
- SSO Moderatörleri (`isCommentModeratorAdmin` bayrağına sahip SSO kullanıcıları - normal SSO kullanıcılarından ayrı olarak faturalandırılır)
- Moderatörler
- Alan Adları

`Creators` ve `Pro` planları için bu metrikler, hesabınızın sınırlarına yaklaşıp yaklaşmadığınızı veya ulaşıp ulaşmadığınızı anlamanıza yardımcı olacak birer kılavuz niteliğindedir.

`Flex` planı için bu sayfa, **bir sonraki** fatura için mevcut kullanımın maliyet dökümünü gösterir. Bu, faturalandırılmadan önce nasıl faturalandırılacağınızı kolayca anlamanızı sağlar. Geçmiş faturaları [Faturalandırma Geçmişi](https://fastcomments.com/auth/my-account/billing-history) bölümünde görebilirsiniz. Her satırın nasıl sayıldığı ve faturanın ne zaman tahsil edildiği [Faturalandırma rehberi](/guide-billing.html#how-flex-usage-is-measured) bölümünde açıklanmıştır.

`Page Loads`, `API Credits` ve `New Comments Stored` metrikleri bir dakikaya kadar gecikmiş olabilir. Diğer metrikler gerçek zamanlıdır.

Bu bilgiler, geçmiş amaçlarıyla bir yıllığına aylık bazda veya tenant'ınızı silene kadar saklanır, ancak arayüz yalnızca geçerli dönemi gösterir.