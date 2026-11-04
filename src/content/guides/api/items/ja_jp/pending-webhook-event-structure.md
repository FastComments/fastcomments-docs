`PendingWebhookEvent` オブジェクトは、保留中のキューに入ったウェブフックイベントを表します。

`PendingWebhookEvent` オブジェクトは自動的に作成され、API経由で手動で作成することはできません。また、1年後に期限切れになります。
削除することで、キューからタスクを取り除くことができます。

イベントの種類は複数あります - `eventType` (`OutboundSyncEventType`) と `type` (`OutboundSyncType`) を確認してください。

このAPIの一般的なユースケースは、カスタムモニタリングの実装です。指定したフィルターに対する未処理の件数を定期的にポーリングするために、`/count` エンドポイントを呼び出すとよいでしょう。

`PendingWebhookEvent` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'PendingWebhookEvent の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress固有の同期タスク。 **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** イベントに関連付けられたコメントid。 **/
    commentId: string
    /** イベント発生時点のコメントオブジェクト。2023年11月から追加を開始しました。 **/
    comment: Comment
    /** コメントに関連付けられている可能性のある外部id。 **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** 初回試行の前、および失敗のたびに設定されます。 **/
    nextAttemptAt: Date
    /** これが作成、削除、更新のいずれのイベントかを示します... **/
    eventType: OutboundSyncEventType
    /** 実行する同期の種類(WordPress、API呼び出しなど)。 **/
    type: OutboundSyncType
    /** コメントに一致したドメイン。このドメインを使用してAPIキーを選択します。 **/
    domain: string
    /** 発生した最後のエラー。この型は型付けされておらず、何が起きたかの「ダンプ」です。通常は statusCode、body、headers マップを含むオブジェクトが入ります。 **/
    lastError: object | null
}
[inline-code-end]