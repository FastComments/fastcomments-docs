## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## Example

[inline-code-attrs-start title = 'getQuestionConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qst-456")
if response.isSome:
  let cfg = response.get()
  discard cfg
[inline-code-end]
