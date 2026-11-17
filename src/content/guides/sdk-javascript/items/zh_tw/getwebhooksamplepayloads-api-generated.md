與 webhook 傳遞格式完全一致的最近留言範例，用於建置整合(例如 Zapier 的範例資料)。每個事件都會傳遞相同的留言物件，因此 `event` 只需為有效值即可。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| event | WebhookEventName | 否 |  |
| limit | number | 否 |  |

## 回應

回傳:[`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getWebhookSamplePayloads 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// 取得特定事件類型的範例酬載
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// 取得所有範例酬載(不使用選填參數)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]