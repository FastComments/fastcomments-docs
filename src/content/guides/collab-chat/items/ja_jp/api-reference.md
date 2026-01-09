### APIの概要

Collab Chat は、テキスト会話をプログラムで管理するための3つのREST APIエンドポイントを提供します。これらのエンドポイントを使用すると、ブラウザウィジェットを使わずにアノテーションの取得、作成、削除が可能です。

これらはブラウザのCookieでユーザーを認証する公開エンドポイントです。APIキーは使用しません。これらのエンドポイントにアクセスするには、ユーザーがブラウザでFastCommentsにログインしている必要があります。

### ベースURL

すべての Collab Chat API エンドポイントは以下の下にあります。

[inline-code-attrs-start title = 'ベースURL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### 認証

これらのエンドポイントはブラウザのCookieでユーザーを認証します。APIキーは使用しません。これらのエンドポイントにアクセスするには、ユーザーがブラウザでFastCommentsにログインしている必要があります。

### すべての会話を取得

特定のページに対するすべてのテキスト会話を取得します。

#### エンドポイント

[inline-code-attrs-start title = 'GETエンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

`urlId` (クエリパラメータ、必須) は会話を取得したいページの識別子です。

#### レスポンス

レスポンスには API のステータス、認証されている場合の現在のユーザーセッション情報、会話の ID、URL、テキスト範囲を含む会話の配列、クリーンな URL 識別子、現在のユーザーがサイト管理者またはモデレーターかどうかを示すフラグ、およびリアルタイム同期のための WebSocket 接続情報（`tenantIdWS`、`urlIdWS`、`userIdWS` を含む）が含まれます。

#### リクエスト例

[inline-code-attrs-start title = 'GETリクエスト例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### レスポンス例

[inline-code-attrs-start title = 'GETレスポンス例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### 会話を作成

特定のテキスト選択に対して新しいテキスト会話を作成します。

#### エンドポイント

[inline-code-attrs-start title = 'POSTエンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

リクエストボディは JSON で、以下の必須フィールドを含める必要があります。

`urlId` (string, 必須) はベースページの識別子です。

`urlIdWithRange` (string, 必須) はテキスト範囲を含む URL で、例: `my-page:p:0:15,0:45{abc123}`。

`pageTitle` (string, 必須) はページのタイトルです。

`selector` (string, 必須) は選択されたテキストを含む要素への DOM パスです。

`range` (string, 必須) はシリアライズされたテキスト範囲で、形式は `startOffset:endOffset,startOffset:endOffset{checksum}` です。

`createdFromCommentId` (string, 必須) はこのチャットを開始したコメントの ID です。

`broadcastId` (string, 必須) はエコー効果を防ぐためのライブ同期用 UUID です。

#### レスポンス

レスポンスには API のステータスと、新しく作成された会話の ID が含まれます。

#### リクエスト例

[inline-code-attrs-start title = 'POSTリクエスト例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### レスポンス例

[inline-code-attrs-start title = 'POSTレスポンス例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### 会話を削除

既存のテキスト会話を削除します。このエンドポイントには管理者またはモデレーターの権限が必要です。もしくは会話が認証ユーザーによって作成されている必要があります。

#### エンドポイント

[inline-code-attrs-start title = 'DELETEエンドポイント'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

`chatId` (パスパラメータ、必須) は削除する会話の ID です。

`broadcastId` (リクエストボディ、必須) はライブ同期用の UUID です。

#### リクエスト例

[inline-code-attrs-start title = 'DELETEリクエスト例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### レスポンス例

[inline-code-attrs-start title = 'DELETEレスポンス例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### レート制限

これらのエンドポイントは標準のFastComments APIのレート制限の対象です。レート制限ミドルウェアはテナントごとに適用され、通常の使用パターンを損なうことなく悪用を防ぎます。

### エラー応答

すべてのエンドポイントは標準的な HTTP ステータスコードを返します。400 レスポンスは無効なリクエストパラメータを示します。401 レスポンスは認証失敗を意味します。403 レスポンスは権限不足を示します。404 レスポンスは会話が見つからないことを意味します。429 レスポンスはレート制限の超過を示します。

エラー応答は詳細を含む JSON ボディを返します:

[inline-code-attrs-start title = 'エラー応答例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### 使用状況の追跡

会話を作成すると `conversationCreateCount` の使用量メトリクスが増加します。すべての WebSocket 同期アクティビティは `pubSubMessageCount` と `pubSubBandwidth` を増加させます。これらのメトリクスは FastComments ダッシュボードの使用状況分析で監視できます。

---