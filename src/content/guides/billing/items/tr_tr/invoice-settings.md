[Billing Info](https://fastcomments.com/auth/my-account/invoice-settings) sayfasındaki `Invoice Settings` düğmesiyle ulaşılabilen Fatura Ayarları sayfası, faturalarınızda neyin görüneceğini ve nereye gönderileceğini kontrol eder. Sayfayı yalnızca `Account Owner` veya bir `Billing Admin` açabilir.

- `Company Name`: gösterge panelinde, e-postalarda ve ekip davetlerinde görünen hesabınızın adı. Bu, fatura adı değildir.
- `Billing Email`: faturaların ve faturalandırma bildirimlerinin gönderileceği adres. Hesap sahibinin bunları almaması gerekiyorsa bunu bir finans veya muhasebe posta kutusuna ayarlayın. Herhangi bir kullanıcının giriş e-postasını değiştirmez.
- `Invoice Currency`: faturalarınızın düzenlendiği para birimi. [Currencies](#currencies) bölümüne bakın.
- `Billing Address`: faturalarda yazdırılan `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` ve `Country` alanları. Aynı alanlar bir kart kaydettiğinizde Billing Info formunda da görünür; ikisi de aynı yere yazar.
- `VAT / Tax ID`: isteğe bağlıdır. Faturalarınızda yazdırılır.

`Save Changes` düğmesine tıklayın. Sayfa `Invoice settings updated successfully.` mesajıyla onaylar ve değişiklikler bir sonraki faturaya uygulanır. Mevcut faturalar değiştirilmez.

### VAT ve Vergi Kimlikleri

Kimliği girmeden önce faturalandırma `Country` ülkenizi seçin, çünkü ülke kimliğin türünü belirler: herhangi bir AB üyesi ülke için AB KDV numarası, Birleşik Krallık için Birleşik Krallık KDV numarası, Avustralya için ABN, Kanada için GST/HST numarası, Amerika Birleşik Devletleri için EIN ve Stripe'ın desteklediği diğer ülkeler için benzer şekilde devam eder.

Kimlik kaydettiğinizde doğrulanır. Seçilen ülke için geçerli değilse, sayfa bunu belirtir ve kimlik kaydedilmez; diğer ayarlarınız ise kaydedilir. Ülkeniz için bir vergi kimliği kaydedemiyorsak, sayfa bunu da size bildirir.

AB faturalandırma adresiniz varsa, Flex faturaları ters yük işlemi olarak %0 KDV ile düzenlenir ve KDV numaranız kaydedildikten sonra üzerinde görünür; böylece muhasebeciniz KDV'yi sizin tarafınızda halledebilir.