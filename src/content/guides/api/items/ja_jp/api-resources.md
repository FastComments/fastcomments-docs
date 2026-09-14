---
### リソース使用量

APIからデータを取得することは、アカウントの使用量としてカウントされることに注意してください。

各リソースは、その使用量をそれぞれのセクションで示します。

リソースによって提供コストが異なります。各エンドポイントには、API呼び出しごとのクレジットの固定コストがあります。一部のエンドポイントでは、オプションやレスポンスサイズに応じてクレジット数が変動します。

API使用量は[請求分析](https://fastcomments.com/auth/my-account/analytics/billing)ページで確認でき、数分ごとに更新されます。

#### 注意！

コメントAPIで`urlId`に渡す値を決定する際の混乱を防ぐため、まずPagesのドキュメントを読むことをお勧めします。

### Webhook

Webhookサブスクリプションには独自のガイドがあります。`POST`、`GET`、`DELETE /api/v1/webhooks`、および`GET /api/v1/webhooks/sample-payloads`は[APIでのサブスクリプション管理](/guide-webhooks.html#webhooks-api-subscriptions)で、イベントペイロードは[Webhook構造](/guide-webhooks.html#webhooks-structures)で文書化されています。

---