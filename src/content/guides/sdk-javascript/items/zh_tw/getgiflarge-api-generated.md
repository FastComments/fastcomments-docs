## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| largeInternalURLSanitized | string | 是 |  |

## 回應

回傳：[`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## 範例

[inline-code-attrs-start title = 'getGifLarge 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a92f4';
const largeInternalURLSanitized: string = 'https://cdn.streamingco.com/gifs/product-demo-large.gif';
let maybeStatus: APIStatus | undefined = undefined; // 選用的元資料（若有）
const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---