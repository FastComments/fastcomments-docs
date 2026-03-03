`UserBadge` は FastComments システムでユーザーに割り当てられたバッジを表すオブジェクトです。

バッジは、ユーザーのアクティビティ（例: コメント数、返信時間、ベテラン状態）に基づいて自動的に割り当てられるか、サイト管理者によって手動で割り当てられます。

`UserBadge` オブジェクトの構造は次の通りです：

[inline-code-attrs-start title = 'UserBadge の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** このユーザーバッジ割り当ての一意の識別子 */
    id: string
    /** このバッジが割り当てられているユーザーのID */
    userId: string
    /** テナントのバッジカタログからのバッジ定義のID */
    badgeId: string
    /** このバッジを作成/割り当てたテナントのID */
    fromTenantId: string
    /** このバッジが作成された時刻（エポックからのミリ秒） */
    createdAt?: number
    /** ユーザーがこのバッジを受け取った時刻（エポックからのミリ秒） */
    receivedAt?: number
    /** 
     * バッジのタイプ: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** 閾値ベースのバッジの場合の閾値の値 */
    threshold?: number
    /** バッジの名前/ラベル */
    name?: string
    /** バッジの詳細説明 */
    description?: string
    /** バッジに表示されるテキスト */
    displayLabel?: string
    /** バッジに表示される画像へのURL */
    displaySrc?: string
    /** バッジの背景色（16進コード） */
    backgroundColor?: string
    /** バッジの境界線色（16進コード） */
    borderColor?: string
    /** バッジのテキスト色（16進コード） */
    textColor?: string
    /** スタイリング用の追加CSSクラス */
    cssClass?: string
    /** ベテランバッジの場合の時間閾値（ミリ秒） */
    veteranUserThresholdMillis?: number
    /** このバッジがユーザーのコメントに表示されるかどうか */
    displayedOnComments: boolean
    /** バッジの表示順序 */
    order?: number
    /** 設定されている場合、このバッジは一致する urlId を持つページにのみ表示されます。グローバルバッジの場合は null。 */
    urlId?: string | null
}
[inline-code-end]
---