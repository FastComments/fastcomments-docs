`Subscription` オブジェクトは、ユーザーのサブスクリプション(購読)を表します。

`Subscription` オブジェクトは、ユーザーがコメントウィジェット内の通知ベルをクリックし、「Subscribe to this page」をクリックしたときに作成されます。

サブスクリプションは API 経由でも作成できます。

`Subscription` オブジェクトが存在すると、その `Subscription` の対象となるページのルートに新しいコメントが投稿された際に、`Notification` オブジェクトが生成され、メールが送信されます。メールの送信可否はユーザーの種類によって異なります。一般ユーザーの場合は `optedInNotifications` に依存し、SSO ユーザーの場合は `optedInSubscriptionNotifications` に依存します。なお、一部のアプリケーションには Web アクセス可能なページという概念がない場合があります。その場合は、購読対象のアイテムの id を `urlId` に設定してください(コメントウィジェットに渡す `urlId` と同じ値です)。

`Subscription` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'Subscription 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO の場合、ユーザー id は `<tenant id>:<user id>` の形式になります。 **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]