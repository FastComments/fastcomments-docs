## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

Returns: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## 範例

[inline-code-attrs-start title = 'getEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate() {
  const tenantId: string = "acme-corp-123";
  const templateId: string = "welcome-email-456";
  const response: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
  const status: APIStatus | undefined = response.status;
  const customTemplate: CustomEmailTemplate | undefined = response.template;
}
[inline-code-end]