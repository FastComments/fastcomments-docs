A `Subscription` object represents a subscription for a user.

`Subscription` objects are created when a user clicks the notification bell in the comment widget and clicks "Bu sayfaya abone ol".

Subscriptions can also be created via the API.

Having a `Subscription` object causes `Notification` objects to be generated, and emails sent, when new comments are left on the root of the associated page
that the `Subscription` is for. Sending of emails depends on the type of user. For regular users this depends on `optedInNotifications`. For SSO Users this depends on `optedInSubscriptionNotifications`. Note that some applications may not have the concept of a web-accessible page, in which case simply set `urlId` to
the id of the item you are subscribing to (same value for `urlId` you would pass to the comment widget).

The structure for the `Subscription` object is as follows:

[inline-code-attrs-start title = 'Abonelik Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO ile, kullanıcı kimliği `<tenant id>:<user id>` formatındadır. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // tarih dizesi
}
[inline-code-end]

---