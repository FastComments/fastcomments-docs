Bir `Subscription` nesnesi, bir kullanıcı için aboneliği temsil eder.

`Subscription` nesneleri, bir kullanıcı yorum bileşenindeki bildirim zilini tıkladığında ve "Bu sayfaya abone ol" seçeneğine tıkladığında oluşturulur.

Abonelikler API üzerinden de oluşturulabilir.

Bir `Subscription` nesnesinin var olması, `Subscription` için ilişkili sayfanın köküne yeni yorumlar bırakıldığında `Notification` nesnelerinin oluşturulmasına ve e-postaların gönderilmesine neden olur. E-postaların gönderilmesi kullanıcı türüne bağlıdır. Normal kullanıcılar için bu, `optedInNotifications` değerine bağlıdır. SSO Kullanıcıları için bu, `optedInSubscriptionNotifications` değerine bağlıdır. Bazı uygulamaların web üzerinden erişilebilir bir sayfa kavramı olmayabilir; bu durumda `urlId` değerini abone olduğunuz öğenin kimliğine (yorum bileşenine ileteceğiniz `urlId` değeriyle aynı) ayarlamanız yeterlidir.

`Subscription` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Subscription Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO ile kullanıcı kimliği `<tenant id>:<user id>` formatındadır. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // tarih dizisi
}
[inline-code-end]