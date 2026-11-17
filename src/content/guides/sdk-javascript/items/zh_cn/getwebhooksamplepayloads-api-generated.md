与 webhook 交付完全相同格式的最近评论，用于构建集成（例如 Zapier 示例数据）。每个事件都交付相同的评论对象，因此 `event` 只需有效即可。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| event | WebhookEventName | 否 |  |
| limit | number | 否 |  |

## 响应

返回：[`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getWebhookSamplePayloads 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// 获取特定事件类型的示例负载
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// 获取所有示例负载（不带可选参数）
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]