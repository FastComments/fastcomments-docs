すべてのエージェントWebhookは、テナントのAPIシークレットを使用したHMAC-SHA256で署名されます。FastCommentsのコメントWebhookと同じ署名方式が使用されます。すでにコメントWebhookを統合している場合、エージェントWebhookは同じ署名ヘッダーと検証フローを再利用します。

### 署名の理由

署名がないと、Webhook URLを知っている攻撃者が、FastCommentsから送信されたように見える偽のイベントをPOSTできてしまいます。署名により、エンドポイントは各配信が本物であることを検証してから処理できます。

### 署名の仕組み

各配信について:

1. プラットフォームは、テナント + マッチしたドメインのAPIシークレットを参照します([Webhooks Overview](#webhooks-overview)を参照)。
2. 現在のUnixタイムスタンプ(ミリ秒)を `X-FastComments-Timestamp` ヘッダーで送信します。
3. `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` を計算し(Stripe方式)、その結果を `sha256=<hex>` として `X-FastComments-Signature` ヘッダーで送信します。
4. エンドポイントはタイムスタンプヘッダーを読み取り、受信した `${timestamp}.${body}` に対してHMACを再計算し、署名ヘッダー内の `sha256=<hex>` 値と比較して、一致しない場合は拒否します。

署名対象のボディは、プラットフォームが送信した**正確なバイト列**に `${timestamp}.` を前置したものです。検証処理では、再シリアライズされたJSON文字列ではなく、生のリクエストボディを使用する必要があります(キーの順序や空白が異なる可能性があるため)。

### APIシークレット

[コメントWebhook](/guide-webhooks.html)で使用されるものと同じAPIシークレットです。これは(tenant, domain)ごとに管理され、テナントのAPI設定で管理されます。シークレットをローテーションする場合は、次の配信より前に、新しい値を読み取るように検証処理を再デプロイしてください。

プラットフォームがマッチしたドメインに対して**APIシークレットが見つからない**場合、配信は行われません。Webhookログには理由「no API secret」として失敗が記録されます。

### 検証例(Node.js)

[inline-code-attrs-start title = 'Webhook署名検証の例'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

タイミング攻撃による署名の漏洩を防ぐため、`===` ではなく `timingSafeEqual` を使用してください。

### 署名対象のボディの内容

完全なエンベロープと、イベント固有の `data` ブロックです。[Webhook Payloads](#webhook-payloads)を参照してください。

### 推奨事項

- **毎回の配信で検証する。** エンドポイントが未署名のリクエストを受け入れる場合、整合性の保証はありません。
- **署名不一致時は拒否する。** 401または403を返してください。不正な署名に対して200 OKを返すと、配信ログで攻撃が見えなくなります。
- **HTTPSを使用する。** 署名は整合性を保護し、TLSは機密性を保護します(シークレットとペイロード内のコメントテキストの両方)。
- **シークレットをローテーションする。** アクセス権を持つチームメンバーが退職した場合や、定期的に実施してください。

### リプレイ対策

署名だけではリプレイ攻撃を防げません。実際の署名済み配信を捕捉した攻撃者が、それを再送信する可能性があります。リプレイ対策はエンドポイント側で行う必要があります:

- `occurredAt` エンベロープフィールドを使用し、たとえば5分より古い配信を拒否します。
- `triggerId` または `approvalId` を重複排除キーとして使用します。すでに処理済みの場合は重複を無視します。

### 関連項目

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Payloads](#webhook-payloads)。
- より広範な署名インフラについては、メインの[Webhooksガイド](/guide-webhooks.html)を参照してください。