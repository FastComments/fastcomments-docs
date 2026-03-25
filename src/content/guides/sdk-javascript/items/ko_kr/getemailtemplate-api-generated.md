## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-marketing-042";
const templateId: string = "tpl_welcome_2026";
const result: GetEmailTemplate200Response = await getEmailTemplate(tenantId, templateId);
const template: CustomEmailTemplate | undefined = result.template;
const subject: string | undefined = template?.subject;
const customParams: CustomConfigParameters | undefined = template?.customConfigParameters;
[inline-code-end]

---