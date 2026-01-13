## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplates 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f4d2b7c';
const responseWithoutSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
const skip: number = 20;
const responseWithSkip: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
[inline-code-end]

---