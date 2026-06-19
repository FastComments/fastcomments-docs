## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-1";
const templateId: string = "welcome-onboard-v2";
const result: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
const status: APIStatus | undefined = result.status;
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject
[inline-code-end]

---