---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_eu_01';
const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
console.log('Email template definitions loaded for', tenantId, templates);
[inline-code-end]

---