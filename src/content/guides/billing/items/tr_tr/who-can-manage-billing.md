Faturalandırma yalnızca iki tür kullanıcıyla sınırlıdır:

- `Account Owner`, yani hesabı oluşturan kullanıcı.
- `Billing Admin` iznine sahip herhangi bir kullanıcı.

Planı değiştirebilen, kart kaydeden veya değiştirebilen, `Invoice Settings` açabilen, Flex harcama limitleri ayarlayabilen ve kredi ekleyebilen tek kullanıcılar onlardır. Deneyen diğer herkes `Only the account owner or a billing admin can change the plan or payment method.` mesajını görür.

Deneme süresi ortası hatırlatması, başarısız ödeme uyarıları ve devre dışı bırakma bildirimleri gibi faturalandırma e-postaları, `Billing Admin` iznine sahip kullanıcılara gönderilir. Bu izni, örneğin finans departmanındaki biri gibi, e-postaları alması gereken kişiye verin; böylece panoya başka erişim ihtiyacı olmadan e-postaları alırlar. Deneme süresi faturalandırma bilgisi olmadan bittikten sonra gönderilen hatırlatmalar, hesaptaki her panoya kullanıcısına gider. Bu e-postaların her birinde abonelikten çıkma bağlantısı bulunur; hesap bildirimlerinden çıkan bir kullanıcı artık bunları almaz.

Faturaların kendileri, tamamen bir panoya kullanıcısına ait olması gerekmeyen [Invoice Settings](#invoice-settings) içindeki `Billing Email` adresine gider.

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) sayfası ise ödeme detayları yerine kullanımı gösterdiği için `Analytics Admin` iznine ihtiyaç duyar.

İzni vermek veya kaldırmak için `Users` sayfasını açın ve kullanıcıyı düzenleyin. Bkz. [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Flex'te her ek yöneticinin faturalandırılan bir koltuk olduğunu unutmayın, bkz. [Admin and Moderator Seats](#admin-and-moderator-seats).

Giriş yapmış olmanıza rağmen `Billing Info` bölümünü hiç göremiyorsanız veya sizi ana sayfaya geri gönderiyorsa, bir panoya kullanıcısı yerine yorumcu hesabıyla giriş yapmışsınızdır. Çıkış yapın ve siteye sahip olan hesapla giriş yapın.