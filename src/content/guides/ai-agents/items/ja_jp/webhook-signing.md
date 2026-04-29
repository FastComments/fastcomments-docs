Every agent webhook is signed with HMAC-SHA256 using your tenant's API secret. The same signing scheme is used for FastComments' comment webhooks - if you have already integrated those, the agent webhooks reuse the same signature header and verification flow.

### なぜ署名するのか

署名が無ければ、攻撃者があなたの webhook URL を知っているだけで、FastComments から送信されたように見える偽のイベントを POST できます。署名があることで、エンドポイントは各配信が真正であることを検証してから処理できます。

### 署名の仕組み

各配信について:

1. The platform looks up the API secret for the tenant + matched domain (see [Webhooks Overview](#webhooks-overview)).
2. It emits the current Unix timestamp (in milliseconds) in the `X-FastComments-Timestamp` header.
3. It computes `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-style) and emits the result as `sha256=<hex>` in the `X-FastComments-Signature` header.
4. Your endpoint reads the timestamp header, recomputes the HMAC over `${timestamp}.${body}` it received, compares to the `sha256=<hex>` value in the signature header, and rejects mismatches.

署名される本文はプラットフォームが送信した**正確なバイト列**で、先頭に `${timestamp}.` が付いています — 検証者は再シリアライズした JSON 文字列ではなく、必ず生のリクエストボディを使う必要があります（キーの順序や空白が異なるためです）。

### API secret

The same API Secret used by [comment webhooks](/guide-webhooks.html). It is per (tenant, domain) and managed in your tenant's API settings. If you rotate the secret, you should re-deploy your verifier to read the new value before the next delivery.

When the platform finds **no API secret** for the matched domain, the delivery does not happen. The webhook log records the failure with reason "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Webhook 署名検証の例'; type='javascript' inline-code-attrs-end]
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

署名の時刻に関する情報漏洩を避けるため、`===` の代わりに `timingSafeEqual` を使用してください。

### 署名された本文に含まれるもの

完全なエンベロープとイベント固有の `data` ブロック。詳しくは [Webhook Payloads](#webhook-payloads) を参照してください。

### 推奨事項

- **すべての配信で検証すること。** エンドポイントが未署名のリクエストを受け入れる場合、整合性の保証がありません。
- **署名が一致しない場合は拒否すること。** 401 または 403 を返してください；不正な署名で 200 OK を返すと配信ログ上で攻撃が隠蔽されます。
- **HTTPS を使用すること。** 署名は整合性を保護し、TLS は機密性（シークレットやペイロード内のコメント本文）を保護します。
- **シークレットをローテーションすること。** アクセス権を持つチームメンバーが離職した場合や、定期的に実施してください。

### 再送（リプレイ）保護

署名だけではリプレイ攻撃を防げません — 実際に署名された配信を攻撃者が取得して再送できてしまいます。リプレイ保護はエンドポイント側で実装してください:

- `occurredAt` エンベロープフィールドを使用し、例えば 5 分以上前の配信を拒否する。
- `triggerId` や `approvalId` をデデュープキーとして使用する — 既に処理済みであれば重複を無視する。

### 参照

- [Webhooks の概要](#webhooks-overview).
- [Webhook ペイロード](#webhook-payloads).
- より広範な署名インフラについてはメインの [Webhooks guide](/guide-webhooks.html) を参照してください。