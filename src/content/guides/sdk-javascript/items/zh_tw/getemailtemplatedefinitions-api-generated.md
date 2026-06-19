## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 回應

回傳: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// 可選參數（如支援）可作為第二個參數傳入，例如 getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]