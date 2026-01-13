A `Subscription` object represents a subscription for a user.

`Subscription` objects are created when a user clicks the notification bell in the comment widget and clicks "Subscribe to this page".

Subscriptions can also be created via the API.

Having a `Subscription` object causes `Notification` objects to be generated, and emails sent, when new comments are left on the root of the associated page
that the `Subscription` is for. Sending of emails depends on the type of user. For regular users this depends on `optedInNotifications`. For SSO Users this depends on `optedInSubscriptionNotifications`. Note that some applications may not have the concept of a web-accessible page, in which case simply set `urlId` to
the id of the item you are subscribing to (same value for `urlId` you would pass to the comment widget).

A `Subscription` オブジェクトはユーザーの購読を表します。

`Subscription` オブジェクトは、ユーザーがコメントウィジェットの通知ベルをクリックし、"このページを購読する" をクリックしたときに作成されます。

購読はAPI経由でも作成できます。

`Subscription` オブジェクトが存在すると、該当するページのルートに新しいコメントが投稿された際に `Notification` オブジェクトが生成され、メールが送信されます。メールの送信はユーザーの種類に依存します。通常のユーザーの場合は `optedInNotifications` に依存します。SSOユーザーの場合は `optedInSubscriptionNotifications` に依存します。ウェブでアクセス可能なページという概念がないアプリケーションもある点に注意してください。その場合は、購読対象のアイテムの id（コメントウィジェットに渡す `urlId` と同じ値）を `urlId` に設定してください。

The structure for the `Subscription` object is as follows:

[inline-code-attrs-start title = 'サブスクリプションの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSOの場合、ユーザーIDは `<tenant id>:<user id>` 形式になります。 **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // 日付文字列
}
[inline-code-end]