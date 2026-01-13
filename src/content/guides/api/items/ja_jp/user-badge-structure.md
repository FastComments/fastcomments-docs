`UserBadge` は FastComments システムでユーザーに付与されるバッジを表すオブジェクトです。

バッジは、ユーザーのアクティビティ（コメント数、返信時間、ベテランステータスなど）に基づいて自動的に付与されるか、サイト管理者によって手動で付与されます。

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'UserBadge の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** このユーザーバッジ割り当ての一意の識別子 */
    id: string
    /** このバッジが割り当てられているユーザーのID */
    userId: string
    /** テナントのバッジカタログにあるバッジ定義のID */
    badgeId: string
    /** このバッジを作成/割り当てたテナントのID */
    fromTenantId: string
    /** このバッジが作成された時刻（エポックからのミリ秒） */
    createdAt?: number
    /** ユーザーがこのバッジを受け取った時刻（エポックからのミリ秒） */
    receivedAt?: number
    /** 
     * バッジの種類: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** 閾値ベースのバッジの場合の閾値 */
    threshold?: number
    /** バッジの名前/ラベル */
    name?: string
    /** バッジの詳細な説明 */
    description?: string
    /** バッジに表示されるテキスト */
    displayLabel?: string
    /** バッジに表示される画像のURL */
    displaySrc?: string
    /** バッジの背景色（16進数コード） */
    backgroundColor?: string
    /** バッジの枠線色（16進数コード） */
    borderColor?: string
    /** バッジのテキスト色（16進数コード） */
    textColor?: string
    /** スタイリング用の追加CSSクラス */
    cssClass?: string
    /** ベテランバッジの場合の時間閾値（ミリ秒） */
    veteranUserThresholdMillis?: number
    /** このバッジがユーザーのコメントに表示されるかどうか */
    displayedOnComments: boolean
    /** バッジの表示順序 */
    order?: number
}
[inline-code-end]
---