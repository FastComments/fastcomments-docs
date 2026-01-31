## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetQuestionConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs200response.nim)

## Example

[inline-code-attrs-start title = 'getQuestionConfigs Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "news-site-001", skip = 0.0)
if response.isSome:
  let configs = response.get()
  discard configs
[inline-code-end]
