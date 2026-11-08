## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplates उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
  const tenantId: string = "acme-corp-123";
  const skip: number = 20;
  const templatesWithSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, skip);
  const templatesWithoutSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
}
[inline-code-end]