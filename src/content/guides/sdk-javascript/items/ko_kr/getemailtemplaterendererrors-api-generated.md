## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| skip | number | 아니요 |  |

## 응답

반환: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_3f6a9b2c-45d8-4e1a";
const templateId: string = "tmpl_9b1c7d6e-22f4-4a3b";
const errorsWithoutSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId);
const errorsWithSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId, 50);
[inline-code-end]

---