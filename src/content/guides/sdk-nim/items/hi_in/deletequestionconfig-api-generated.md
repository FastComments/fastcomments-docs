## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'deleteQuestionConfig उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResult, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456")
if apiResult.isSome:
  let empty = apiResult.get()
[inline-code-end]