## פרמטרים

| שם | Type | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| largeInternalURLSanitized | string | כן |  |

## תגובה

מחזיר: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const largeInternalURLSanitized: string = '/assets/internal/gifs/launch-party-9f8b7c.gif';
const correlationId?: string = 'req-20260619-01';
const result: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---