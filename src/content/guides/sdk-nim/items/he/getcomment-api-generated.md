## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[GetComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-987654321")
if response.isSome:
  let comment = response.get()
  echo comment
[inline-code-end]

---