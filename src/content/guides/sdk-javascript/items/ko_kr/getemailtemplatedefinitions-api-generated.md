---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// 선택적 매개변수(지원되는 경우)는 두 번째 인수로 전달할 수 있습니다. 예: getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---