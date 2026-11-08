## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'updateQuestionConfig उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.updateQuestionConfig(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionConfigBody = UpdateQuestionConfigBody()
)

if apiResp.isSome:
  let resp = apiResp.get()
[inline-code-end]