## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |

## תגובה

מחזיר: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomment​s-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑getV1PageLikes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let pageLikes = response.get()
  # השתמש ב‑pageLikes לפי הצורך
[inline-code-end]