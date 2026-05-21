## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| largeInternalURLSanitized | string | כן |  |

## תשובה

מחזיר: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifGetLargeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGifLarge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a92f4';
const largeInternalURLSanitized: string = 'https://cdn.streamingco.com/gifs/product-demo-large.gif';
let maybeStatus: APIStatus | undefined = undefined; // מטא-נתונים אופציונליים אם זמינים
const response: GifGetLargeResponse = await getGifLarge(tenantId, largeInternalURLSanitized);
[inline-code-end]

---