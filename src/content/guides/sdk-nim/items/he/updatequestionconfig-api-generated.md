## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456", updateQuestionConfigBody = default(UpdateQuestionConfigBody))
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---