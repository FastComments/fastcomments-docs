## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createEmailTemplateBody | CreateEmailTemplateBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createEmailTemplate उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const templateBody: CreateEmailTemplateBody = {
  name: "Weekly Summary",
  subject: "Your weekly activity report",
  // वैकल्पिक फ़ील्ड
  replyTo: "no-reply@myapp.com",
  htmlContent: "<p>Hello \{{userName}}, here is your summary...</p>"
};

const response: CreateEmailTemplateResponse = await createEmailTemplate(tenantId, templateBody);

console.log(response.template.id);
[inline-code-end]