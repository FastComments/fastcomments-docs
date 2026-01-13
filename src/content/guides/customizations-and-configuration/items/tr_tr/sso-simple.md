---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO ile, yorum aracıına kullanıcı hakkında bilgi sağlayabiliriz; böylece kullanıcılar yorum yapmak için kullanıcı adlarını veya e‑postalarını girmek zorunda kalmazlar.

Şu şekilde Simple SSO'yu yapılandırabiliriz:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Kullanıcı oturum açmış olacaktır ve arka planda bir SSO Kullanıcısı oluşturulacaktır. Kullanıcı API'den alınmışsa `createdFromSimpleSSO` değeri `true` olarak ayarlanmış olacaktır.

Notlar: 

- E‑posta, Simple SSO için benzersiz tanımlayıcıdır.
- Simple SSO ile bir e‑posta sağlamak zorunlu değildir; ancak varsayılan olarak yorumları "Doğrulanmamış" olarak görünecektir. <b>Eğer e‑posta sağlanmazsa, kullanıcı tam olarak kimlik doğrulanamaz.</b>
- **YENİ** Ocak 2022'den itibaren: Kullanıcı adlarının fastcomments.com genelinde benzersiz olması gerekmemektedir
- Eğer bir e‑posta sağlanmışsa ve kullanıcı orijinal olarak Secure SSO'dan oluşturulmamışsa, Simple SSO otomatik olarak SSO kullanıcıları oluşturup güncelleyebilir.
- Kullanıcı için rozetleri `badgeConfig` özelliğiyle belirtebilirsiniz. `badgeIds` dizisi, kullanıcıyla ilişkilendirilecek rozetlerin kimliklerini içerir. `override` `true` olarak ayarlanmışsa yorumlarda gösterilen mevcut rozetlerin tamamının yerine koyar; `false` ise mevcut rozetlere ekler.

---