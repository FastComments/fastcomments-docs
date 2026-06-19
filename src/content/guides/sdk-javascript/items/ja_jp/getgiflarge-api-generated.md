---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| largeInternalURLSanitized | string | はい |  |

## レスポンス

返却値: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## 例

[inline-code-attrs-start title = 'getGifLarge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const largeInternalURLSanitized: string = '/assets/internal/gifs/launch-party-9f8b7c.gif';
const correlationId?: string = 'req-20260619-01';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---