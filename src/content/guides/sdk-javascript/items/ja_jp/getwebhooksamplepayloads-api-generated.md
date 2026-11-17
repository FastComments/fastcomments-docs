Webhook配信とまったく同じ形式の最新コメント。統合を構築するためのものです(例: Zapierの
サンプルデータ)。すべてのイベントで同じコメントオブジェクトが配信されるため、`event` は有効であるだけで構いません。

## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| event | WebhookEventName | いいえ |  |
| limit | number | いいえ |  |

## レスポンス

戻り値: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## 例

[inline-code-attrs-start title = 'getWebhookSamplePayloads の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// 特定のイベントタイプのサンプルペイロードを取得
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// すべてのサンプルペイロードを取得(オプションパラメーターなし)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]