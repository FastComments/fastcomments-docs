`Notification`オブジェクトはユーザーのための通知を表します。

`Notification`オブジェクトは自動的に作成され、API経由で作成することはできません。これらはまた1年後に期限切れになります。

通知は削除できません。ただし、`viewed`を`false`に設定するよう更新することはでき、`viewed`でクエリすることができます。

ユーザーは、通知の`optedOut`を`true`にすることで特定のコメントの通知をオプトアウトできます。再度受け取りたい場合は`false`に設定してください。

通知にはさまざまなタイプがあります — `relatedObjectType`と`type`を確認してください。

通知の作成方法は非常に柔軟で、多くのシナリオでトリガーされます（`NotificationType`を参照）。

現在、`Notification`の存在がメールが送信される、または送信されるべきであることを意味するわけではありません。むしろ、通知は通知フィードおよび関連する統合で使用されます。

`Notification`オブジェクトの構造は次のとおりです：

[inline-code-attrs-start title = '通知の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** 誰かがあなたに返信した場合。 **/
    RepliedToMe = 0,
    /** あなたがコメントしたスレッド内のどこか（子の子も含む）で誰かが返信した場合。 **/
    RepliedTransientChild = 1,
    /** あなたのコメントがアップボートされた場合。 **/
    VotedMyComment = 2,
    /** あなたが購読しているページのルートに新しいコメントが残された場合。 **/
    SubscriptionReplyRoot = 3,
    /** 誰かがあなたのプロフィールにコメントした場合。 **/
    CommentedOnProfile = 4,
    /** ダイレクトメッセージ（DM）がある場合。 **/
    DirectMessage = 5,
    /** TrialLimitsはテナントユーザーのみ対象です。 **/
    TrialLimits = 6,
    /** あなたが@メンションされた場合。 **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** SSOを使用する場合、ユーザーIDの形式は `<tenant id>:<user id>`です。 **/
    userId?: string
    /** SSOを扱う際は、`userId`のみ気にすればよいです。 **/
    anonUserId?: string
    /** urlIdはほとんどの場合定義されています。テナントレベルの通知の場合のみ省略可能で、これは稀です。 **/
    urlId?: string
    /** URLは通知のソースへ迅速に移動できるようキャッシュされます。 **/
    url?: string
    /** ページタイトルは通知のソースを素早く確認できるようキャッシュされます。 **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** 例えば、コメントID。 **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // 日付文字列
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserNameとfromUserAvatarSrcは通知の表示を迅速に行うためにここでキャッシュされます。ユーザーが更新されるとこれらも更新されます。 **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** これをtrueに設定するとこのオブジェクトの通知を受け取らなくなります。 **/
    optedOut?: boolean
}
[inline-code-end]