## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| errorId | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeRenderError() {
  const tenantId: string = "acme-corp-tenant";
  const templateId: string = "welcome-email-template";
  const errorId: string = "render-err-20230915";

  const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
}
[inline-code-end]