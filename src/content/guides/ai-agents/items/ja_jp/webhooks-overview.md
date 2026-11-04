エージェントWebhookは、エージェントの実行が完了したときや承認の状態が変化したときにプラットフォームが発行するHTTPコールバックです。これらを使用して、エージェントのアクティビティをモデレーションダッシュボード、監査ログ、Slackチャンネル、エスカレーションツールなど、独自のシステムに転送できます。

[AI Agentsページ](https://fastcomments.com/auth/my-account/ai-agents)の**Webhooks**タブで設定します。

### 配信される内容

4種類のイベントタイプがあります:

- **`trigger.succeeded`** - エージェントの実行が正常に完了した。
- **`trigger.failed`** - エージェントの実行でエラーが発生した。
- **`approval.requested`** - アクションが人間による承認のためにキューに入った。
- **`approval.decided`** - 承認が承認、却下、または実行失敗された。

どのイベントがいつ発火するかについては[Webhook Events](#webhook-events)を、それぞれのスキーマについては[Webhook Payloads](#webhook-payloads)を参照してください。

### 配信されない内容

- **ツールアクションごとのWebhook。** `pin_comment`を呼び出す実行は、ピン操作に対して個別のWebhookを発火しません。そのアクションは実行の`trigger.succeeded`ペイロードに含まれます。アクションごとの配信が必要な場合は、トリガーペイロードの`actions`配列をパースしてください。
- **ドロップされたトリガー。** ディスパッチされないトリガー(予算超過、スコープ不一致)はWebhookを発火しません。ドロップは[Analyticsページ](#analytics-page)でのみ確認できます。
- **リプレイによって生成されたトリガー。** テスト実行はWebhookを発火しません。[Test Runs (Replays)](#test-runs-replays)を参照してください。

### 設定

各Webhookで以下を設定します:

- **URL** - POST先のHTTPSエンドポイント。
- **Domain** - このWebhookが適用されるコメントのドメイン(テナントは複数のドメインをホストしている場合があります)。`*`はすべてのドメインに一致し、`*.example.com`はglobパターン、完全一致のドメインはそのドメインのみに一致します。
- **Events** - 購読する4種類のイベントタイプ。
- **Agents** - 「すべてのエージェント」の場合は空、またはフィルターする特定のエージェントIDのリスト。
- **Method** - POSTまたはPUT(デフォルトはPOST)。
- **No-retry status codes** - 最終的な失敗として扱われ、リトライされないHTTPレスポンスコード(例: 410 Gone、422 Unprocessable)。[Webhook Retries](#webhook-retries)を参照してください。

複数のWebhookが同じイベントを購読できます。それぞれが独立してキューに入り、各自のURLに配信されます。

### ドメインごとのマッチング

特定のイベントは、**コメントのドメインに`domain`フィールドが一致するすべてのWebhook**に配信されます。マッチングにはシンプルなglobを使用します:

- 完全一致: `example.com`は`example.com`のみに一致します。
- ワイルドカードスター: `*`はすべてのドメインに一致します。
- サブドメインglob: `*.example.com`は`blog.example.com`や`forum.example.com`に一致しますが、`example.com`自体には一致しません。

ペイロード上のドメインは、コメントの`domain`、テナントのドメイン設定に対してパースされたURL、または`urlId`による`Page`ルックアップのうち、最初の非nullの結果です。

### エージェントごとのフィルタリング

**Agents**フィールドにより、Webhookは特定のエージェントのみを購読できます。空の場合は「すべてのエージェント」を意味します。空でない場合、Webhookはリスト内のエージェントに対してのみ発火します。

これは、モデレーションイベント用のWebhookとエンゲージメントイベント用のWebhookを別々に持ち、それぞれ異なる下流システムにルーティングする場合に便利です。

### テスト送信

Webhook設定UIには**Test send**ボタンがあり、実際のイベントを待たずに、接続性、署名、エンドポイントのレスポンスコードを確認できるよう、偽のペイロードをURLにPOSTします。

### 配信ログ

すべての配信(およびすべてのリトライ)は[Webhook Delivery Logs](#webhook-logs)ページに記録されるため、ネットワーク上で何が起こったかを確認できます。

### 署名

すべてのWebhookは、テナントのAPIシークレットを使用したHMAC-SHA256で署名されます。[Webhook Signing](#webhook-signing)を参照してください。

### 利用条件

エージェントWebhookには、テナントの有効な課金情報が必要です。既存のコメントWebhookと同じ署名/シークレットインフラストラクチャを使用します。コメントWebhookをすでに統合している場合([Webhooksガイド](/guide-webhooks.html)を参照)、エージェントWebhookの統合は新しいイベントタイプが追加された同じ構造です。