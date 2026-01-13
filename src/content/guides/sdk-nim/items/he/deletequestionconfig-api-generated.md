## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-456")
if response.isSome:
  let respVal = response.get()
  echo "Delete succeeded for tenant my-tenant-123"
else:
  echo "Delete returned no data (status: ", $httpResponse.status, ")"
[inline-code-end]

---