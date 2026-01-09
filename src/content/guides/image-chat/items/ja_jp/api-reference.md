### APIの概要

Image Chat は、画像の会話をプログラムで管理するための3つのREST APIエンドポイントを提供します。これらのエンドポイントを使用すると、ブラウザウィジェットを使用せずにマーカーの取得、作成、削除ができます。

すべてのAPIエンドポイントは認証を必要とし、標準のFastComments APIパターンに従います。これらはAPIキーではなくブラウザのクッキーで認証する公開エンドポイントです。

### ベースURL

すべての Image Chat API エンドポイントは次の下にあります:

```
https://fastcomments.com/comment-image-chats
```

### 認証

これらのエンドポイントはブラウザのクッキーを介してユーザーを認証します。APIキーは使用しません。ユーザーはブラウザでFastCommentsにログインしている必要があります。

### すべての会話を取得

特定の画像に対するすべての画像会話を取得します。

#### エンドポイント

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

`urlId` (クエリパラメータ、必須) は会話を取得したい画像の識別子です。

#### レスポンス

レスポンスにはAPIステータス、認証されている場合の現在のユーザーセッション情報、ID、URL、X/Y座標を含む会話の配列、クリーンなURL識別子、現在のユーザーがサイト管理者またはモデレーターかを示すフラグ、およびライブ同期用のWebSocket接続情報（`tenantIdWS`、`urlIdWS`、`userIdWS`）が含まれます。

#### リクエスト例

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### レスポンス例

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### 会話の作成

画像上の特定の位置に対して新しい画像会話を作成します。

#### エンドポイント

```
POST /comment-image-chats/:tenantId
```

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

リクエストボディはJSONで、以下の必須フィールドを含める必要があります。

`urlId` (string, 必須) は基本のページ識別子です。

`windowUrlId` (string, 必須) は画像ソースと座標を組み合わせたURLで、例えば `my-page:/images/photo.jpg:25.5:30.2` のようになります。

`pageTitle` (string, 必須) はページのタイトルです。

`src` (string, 必須) は画像のソースURLです。

`x` (number, 必須) はパーセンテージ（0-100）としてのX座標です。

`y` (number, 必須) はパーセンテージ（0-100）としてのY座標です。

`createdFromCommentId` (string, 必須) はこのチャットを開始したコメントのIDです。

`broadcastId` (string, 必須) はエコー効果を防ぐためのライブ同期用UUIDです。

#### レスポンス

レスポンスにはAPIステータスと、新しく作成された会話のIDが含まれます。

#### リクエスト例

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### レスポンス例

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### 会話の削除

既存の画像会話を削除します。このエンドポイントを使用するには管理者またはモデレーター権限が必要です。もしくは会話が認証されたユーザーによって作成されている必要があります。

#### エンドポイント

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### パラメータ

`tenantId` (パスパラメータ、必須) はあなたの FastComments テナントIDです。

`chatId` (パスパラメータ、必須) は削除する会話のIDです。

`broadcastId` (リクエストボディ、必須) はライブ同期用のUUIDです。

#### リクエスト例

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### レスポンス例

```json
{
  "status": "success"
}
```

### 座標系

X および Y の座標は画像の寸法に対するパーセンテージとして保存されます。X は左端からの水平位置を示します（0 = 左端、100 = 右端）。Y は上端からの垂直位置を示します（0 = 上端、100 = 下端）。

これらのパーセンテージ値は精度のために小数を含むことができます。例えば、x: 25.5 は画像の左端から25.5%の位置を意味します。

### レート制限

これらのエンドポイントは標準のFastComments APIのレート制限の対象です。レート制限ミドルウェアはテナントごとに適用され、通常の使用パターンを妨げることなく悪用を防ぎます。

### エラー応答

すべてのエンドポイントは標準のHTTPステータスコードを返します。400は無効なリクエストパラメータを示します。401は認証失敗を意味します。403は権限不足を示します。404は会話が見つからないことを意味します。429はレート制限を超えたことを示します。

エラー応答には詳細を含むJSONボディが含まれます:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### 使用状況の追跡

会話を作成すると `conversationCreateCount` の使用状況メトリクスが増加します。すべてのWebSocket同期アクティビティは `pubSubMessageCount` と `pubSubBandwidth` を増加させます。これらのメトリクスはFastCommentsダッシュボードの使用状況分析で監視できます。