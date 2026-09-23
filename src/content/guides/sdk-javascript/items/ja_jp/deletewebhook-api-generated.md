---
購読解除 (REST フックの購読解除)。この API を通じて作成されたサブスクリプションのみがここで削除でき
ます; ダッシュボードで管理されている Webhook はダッシュボードで編集されます。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteWebhook の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]

---