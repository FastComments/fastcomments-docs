URL をコメントイベントにサブスクライブします（REST フックのサブスクライブ）。同じ URL を同じイベントとドメインに再度サブスクライブすると、既存のサブスクリプションが返されます。配信は HMAC 署名されます。webhooks ガイドをご参照ください。レガシー `token` ヘッダーは API サブスクリプションには送信されません。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createWebhookParams | CreateWebhookParams | Yes |  |

## Response

返却: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'createWebhook の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // ペイロード検証用のオプションシークレット
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]