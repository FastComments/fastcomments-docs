## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qst-456")
if response.isSome:
  let cfg = response.get()
  discard cfg
[inline-code-end]

---