## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplates 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a9c';
const templatesPage1: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
const templatesPage2: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, 25);
console.log(templatesPage1, templatesPage2);
[inline-code-end]

---