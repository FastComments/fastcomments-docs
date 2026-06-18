## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| largeInternalURLSanitized | string | 예 |  |

## 응답

반환: [`GetGifLarge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifLarge200Response.ts)

## 예제

[inline-code-attrs-start title = 'getGifLarge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const largeInternalURLSanitized: string = "https://cdn.fastcomments.com/gifs/07d3f6_large.gif";
const preferWebP: boolean | undefined = true; // 선택적 선호
const urlToUse: string = preferWebP ? largeInternalURLSanitized.replace(".gif", ".webp") : largeInternalURLSanitized;
const response: GetGifLarge200Response = await getGifLarge(tenantId, urlToUse);
[inline-code-end]

---