## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tag | string | לא |  |
| tenantId | string | כן |  |
| updateHashTagBody | UpdateHashTagBody | לא |  |

## תגובה

מחזיר: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]