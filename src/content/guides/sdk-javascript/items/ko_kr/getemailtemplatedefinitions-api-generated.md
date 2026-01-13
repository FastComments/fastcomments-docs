## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-78f2';
const emailTemplates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
[inline-code-end]

---