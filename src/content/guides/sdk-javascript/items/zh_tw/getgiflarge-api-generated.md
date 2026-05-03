## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| largeInternalURLSanitized | string | 是 |  |

## 回應

回傳: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## 範例

[inline-code-attrs-start title = 'getGifLarge 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_marketing_tenant_7';
const largeInternalURLSanitized: string = 'https://cdn.acmeinc.com/gifs/promo-spring-2026_large_sanitized.gif';
const includePreview: boolean | undefined = undefined; // 呼叫者可能會使用的可選旗標
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
console.log(result, includePreview);
[inline-code-end]