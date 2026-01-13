`UserBadgeProgress` は、FastComments システムでユーザーがさまざまなバッジを獲得するための進捗を表すオブジェクトです。

このトラッキングは、ユーザーの活動やコミュニティへの参加に基づいていつ自動的にバッジを付与するべきかを判断するのに役立ちます。

以下が `UserBadgeProgress` オブジェクトの構造です：

[inline-code-attrs-start title = 'UserBadgeProgress の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** この進捗レコードの一意の識別子 */
    id: string
    /** この進捗レコードが属するテナントのID */
    tenantId: string
    /** この進捗レコードが追跡するユーザーのID */
    userId: string
    /** システム内でのユーザーの最初のコメントのID */
    firstCommentId?: string
    /** ユーザーの最初のコメントの日付（エポックからのミリ秒） */
    firstCommentDate?: number
    /** ユーザーの活動に基づいて自動計算される信頼度 */
    autoTrustFactor?: number
    /** 管理者によって手動で設定される信頼度 */
    manualTrustFactor?: number
    /** さまざまな指標を含む詳細な進捗オブジェクト。キーは BadgeType 列挙型と一致します */
    progress: {
        /** 0: CommentCount - ユーザーが行ったコメントの数 */
        '0'?: number
        /** 1: CommentUpVotes - ユーザーが受け取ったアップボートの数 */
        '1'?: number
        /** 2: CommentReplies - ユーザーが行った返信の数 */
        '2'?: number
        /** 3: CommentsPinned - ユーザーが持つピン留めされたコメントの数 */
        '3'?: number
        /** 4: Veteran - ユーザーのアカウントの経過期間 */
        '4'?: number
        /** 5: NightOwl - 夜間の時間帯にユーザーが投稿した回数 */
        '5'?: number
        /** 6: FastReplyTime - 平均返信時間（ミリ秒） */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - モデレーターバッジ用：削除したコメントの数 */
        '7'?: number
        /** 8: ModeratorCommentsApproved - モデレーターバッジ用：承認したコメントの数 */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - モデレーターバッジ用：非承認にしたコメントの数 */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - モデレーターバッジ用：レビューしたコメントの数 */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - モデレーターバッジ用：スパムとしてマークしたコメントの数 */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - モデレーターバッジ用：スパムではないとマークしたコメントの数 */
        '12'?: number
        /** 13: RepliedToSpecificPage - 各ページごとの返信数 */
        '13'?: Record<string, number>
    }
}
[inline-code-end]