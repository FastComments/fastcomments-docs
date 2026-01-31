## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | No |  |

## Response

Returns: [`Option[CreateQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config200response.nim)

## Example

[inline-code-attrs-start title = 'createQuestionConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionConfig(tenantId = "my-tenant-123", createQuestionConfigBody = CreateQuestionConfigBody())
if response.isSome:
  let createdConfig = response.get()
  discard createdConfig
else:
  echo "createQuestionConfig returned no data; HTTP status: ", httpResponse.status()
[inline-code-end]
