## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| updateQuestionResultBody | UpdateQuestionResultBody | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'updateQuestionResult उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateQuestionResult(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionResultBody = UpdateQuestionResultBody()
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]