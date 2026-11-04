[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO ile, yorum widget'ına kullanıcı hakkında bilgi sağlayabiliriz, böylece kullanıcı yorum yapmak için kullanıcı adı veya e-posta girmek zorunda kalmaz.

Simple SSO'yu aşağıdaki gibi yapılandırabiliriz:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Kullanıcı oturum açmış olacak ve sahne arkasında bir SSO Kullanıcısı oluşturulacak. Kullanıcı, API'den alındıysa `createdFromSimpleSSO` özelliği `true` olarak ayarlanmış olacaktır.

Notlar: 

- E-posta, Simple SSO için benzersiz tanımlayıcıdır.
- Simple SSO ile e-posta sağlamak zorunlu değildir, ancak varsayılan olarak yorumları "Doğrulanmamış" olarak gösterilir. <b>E-posta sağlanmazsa, kullanıcı tam olarak kimlik doğrulaması yapılamaz.</b>
- **YENİ** Ocak 2022'den beri: Kullanıcı adlarının fastcomments.com genelinde benzersiz olması gerekmez
- Simple SSO, bir e-posta sağlanmışsa ve kullanıcı başlangıçta Secure SSO'dan oluşturulmamışsa, SSO kullanıcılarını otomatik olarak oluşturabilir ve güncelleyebilir. Simple SSO yükü, Secure SSO veya API tarafından oluşturulan bir kullanıcıyı adlandırıyorsa, o kullanıcı dokunulmaz kalır ve ziyaretçi ayrı bir kullanıcı alır.
- Kullanıcı için rozetleri `badgeConfig` özelliğiyle belirtebilirsiniz. `badgeIds` dizisi, kullanıcıyla ilişkilendirilecek global rozetlerin kimliklerini içerir. `pageBadgeIds` dizisi, mevcut sayfaya (`urlId`) özgü rozet kimliklerini içerir — bu rozetler yalnızca atandıkları sayfada gösterilir. `override` `true` olarak ayarlanırsa, mevcut gösterilen rozetleri (global ve sayfa kapsamlı olanlar bağımsız olarak geçersiz kılınır) değiştirir; `false` ise mevcut rozetlere ekler.

#### Simple SSO Ayarları

Simple SSO her zaman kullanılabilir. İki ayar, FastComments'un ona ne kadar güveneceğini kontrol eder. Bunları <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> içinde SSO aratarak bulabilirsiniz. Her ikisi de varsayılan olarak kapalıdır ve "Yorumları Görmek İçin SSO Gereksin" açık olsun ya da olmasın uygulanır.

Bu ayarlar eklendiğinde zaten Simple SSO kullanan hesaplarda her iki ayar da açık olarak ayarlandı, bu yüzden onlar için bir değişiklik olmadı.

Yalnızca bir sayfaya en iyi uyan özelleştirme kuralı uygulanır, bu yüzden birden fazla kuralınız varsa, bunları her birine ayarlayın. Diğer kurallarınızdan herhangi biri bu ayarları açık tutuyorsa, yeni bir kural da bunlarla başlar.

##### Simple SSO ile Yorumları Otomatik Doğrulama

Bu açık olduğunda, Simple SSO kullanıcılarından gelen yorumlar hemen doğrulanır.

Bu kapalı olduğunda, Simple SSO kullanıcılarından gelen yorumlar, kullanıcı Simple SSO yükündeki adrese gönderdiğimiz bir bağlantıya tıklayana kadar "Doğrulanmamış" olarak gösterilir. Yalnızca doğrulanmış yorumları otomatik onaylıyorsanız, bu yorumlar da o bağlantıyı bekler. Yükte e-posta yoksa, yorum doğrulanmamış olarak kalır.

Bu ayar yalnızca yorumları etkiler. Simple SSO kullanıcılarından gelen oylar her zaman hemen sayılır.

##### Simple SSO Yöneticilerine İzin Ver

Bu açık olduğunda, bir Simple SSO kullanıcısı hesabınızın yöneticisi veya moderatörü olabilir:

- Yönetici veya moderatör izinlerine sahip bir Simple SSO kullanıcısı bu izinleri korur.
- Yöneticilerinizden veya moderatörlerinizden birinin aynı e-posta adresine sahip bir Simple SSO kullanıcısı, onların izinlerini alır.

Bu kapalı olduğunda, Simple SSO bu izinleri vermez. Bu izinlere sahip bir kullanıcıyı adlandıran Simple SSO yükü, izinleri olmayan ayrı bir kullanıcı oluşturur ve orijinal kullanıcı dokunulmaz kalır.

Simple SSO yükleri imzalanmamıştır, bu yüzden bu açık olduğunda, bir yöneticinin e-posta adresini veya kullanıcı kimliğini bilen herkes onun gibi davranabilir. Bu siteniz için önemliyse, bu ayarı kapalı tutun ve yöneticileriniz için Secure SSO kullanın.

Secure SSO veya API tarafından oluşturulan kullanıcılar, Simple SSO tarafından erişilemez; bu ayar açık olsun ya da olmasın.

---