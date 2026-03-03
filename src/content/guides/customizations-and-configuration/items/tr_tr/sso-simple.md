[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO ile yorum bileşenine, kullanıcıların yorum yapmak için kullanıcı adlarını veya e-postalarını girmelerine gerek kalmaması için kullanıcı hakkında bilgi sağlayabiliriz.

Simple SSO'yi şu şekilde yapılandırabiliriz:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Kullanıcı giriş yapmış olacak ve arka planda bir SSO Kullanıcısı oluşturulacak. Kullanıcı, API'den alındıysa `createdFromSimpleSSO` `true` olarak ayarlanmış olacaktır.

Notlar: 

- E-posta, Simple SSO için benzersiz tanımlayıcıdır.
- Simple SSO ile e-posta sağlamak zorunlu değildir; ancak varsayılan olarak yorumları "Doğrulanmamış" olarak görünecektir. <b>Eğer e-posta sağlanmazsa, kullanıcı tamamen doğrulanamaz.</b>
- **YENİ** 2022 Ocak ayından itibaren: kullanıcı adlarının fastcomments.com genelinde benzersiz olması gerekmiyor
- Eğer e-posta sağlanmışsa ve kullanıcı başlangıçta Secure SSO'dan oluşturulmadıysa, Simple SSO SSO kullanıcılarını otomatik olarak oluşturabilir ve güncelleyebilir.
- Kullanıcı için rozetleri `badgeConfig` özelliği ile belirtebilirsiniz. `badgeIds` dizisi, kullanıcıyla ilişkilendirilecek global rozetlerin ID'lerini içerir. `pageBadgeIds` dizisi, mevcut sayfaya (`urlId`) özel rozet ID'lerini içerir — bu rozetler yalnızca atandıkları sayfada gösterilir. Eğer `override` `true` olarak ayarlanırsa, mevcut gösterilen rozetlerin yerine geçer (global ve sayfaya özel rozetler bağımsız olarak üzerine yazılır); eğer `false` ise mevcut rozetlere ekler.