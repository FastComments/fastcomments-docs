Her ücret, ister plan yenileme, ister Flex ayı, ister plan değişikliği olsun, bir fatura oluşturur. Faturalar, ödendikten sonra makbuz olarak da işlev görür.

### Faturaları Bulma

[Billing History](https://fastcomments.com/auth/my-account/billing-history) sayfasını açın veya Billing Info sayfasında `View Invoices` bağlantısına tıklayın. Tablo, her faturayı tarihi, faturalandırma dönemi, fatura numarası, açıklaması, toplamı, durumu (`Paid`, `Unpaid`, `Draft` veya `Void`) ve son ödeme tarihiyle birlikte listeler; sayfa başına on fatura, en yeniden eskiye doğru sıralanır.

Her fatura için:

- `View`, faturayı Stripe'ın barındırılan sayfasında açar; buradan indirebilirsiniz de.
- `PDF`, PDF'i doğrudan indirir.
- `Pay Now`, ödenmemiş bir faturada `View` yerine görünür ve sizi o faturanın Stripe ödeme sayfasına götürür; buradan doğrudan ödeme yapabilirsiniz.

Faturalar ayrıca oluşturulduklarında faturalandırma e-posta adresinize e-posta ile gönderilir. Bunların hesap sahibi yerine bir finans posta kutusuna gitmesini istiyorsanız, [Invoice Settings](#invoice-settings) bölümünde `Billing Email` alanını ayarlayın.

### Faturada Neler Var

Fatura, [Invoice Settings](#invoice-settings) bölümündeki faturalandırma adını ve adresini ve KDV / Vergi Kimliğini, kalem öğelerini, uygulanan varsa önceden ödenmiş [krediyi](#credits) ve kartınıza tahsil edilen tutarı gösterir. Flex faturaları her kullanım türü için bir satır listeler, bkz. [Flex Kullanımı Nasıl Ölçülür](#how-flex-usage-is-measured). Avrupa Birliği'ndeki faturalandırma adresleri için Flex faturaları %0 KDV ile ve `Reverse Charge Transaction: Customer to pay VAT` alt bilgisiyle düzenlenir.

Fatura ayarlarındaki değişiklikler, kaydettikten sonra oluşturulan faturalara uygulanır. Zaten düzenlenmiş bir fatura düzenlenemez; bu nedenle şirket bilgilerinizi ve KDV / Vergi Kimliğinizi bir sonraki faturalandırma tarihinden önce ayarlayın.

### Başka Yerde Faturalandırılan Hesaplar

Faturalandırmanız harici olarak yönetiliyorsa, örneğin destek ile düzenlenmiş bir fatura anlaşmasıyla, faturalar karttan tahsil edilmek yerine ödemeniz için size gönderilir ve Billing History sayfasında görünmeyebilir. Shopify veya BigCommerce üzerinden kurulan hesaplar o platform tarafından faturalandırılır ve Billing Info sayfası oradaki plan sayfasına bağlantı verir. Bir ana hesap üzerinden faturalandırılan alt hesapların kendi faturaları yoktur; bkz. [Ne Zaman Ücretlendirilirsiniz](#when-you-are-charged).