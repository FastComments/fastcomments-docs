## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createQuestionConfigBody | CreateQuestionConfigBody | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'createQuestionConfig उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let configBody = CreateQuestionConfigBody()
let (maybeResp, httpResp) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = configBody,
)
if maybeResp.isSome:
  let resp = maybeResp.get()
  # resp का आवश्यकतानुसार उपयोग करें
[inline-code-end]