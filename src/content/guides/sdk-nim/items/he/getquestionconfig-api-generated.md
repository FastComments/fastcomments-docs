## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## דוגמה

[inline-code-attrs-start title = 'getQuestionConfig דוגמה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (questionConfigOpt, httpResp) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "question-987")
if questionConfigOpt.isSome:
  let config = questionConfigOpt.get()
  echo config
[inline-code-end]