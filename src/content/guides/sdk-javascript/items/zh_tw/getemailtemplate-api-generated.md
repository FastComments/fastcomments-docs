---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

返回：[`GetEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate(): Promise<void> {
    const tenantId: string = "tenant-12345";
    const templateId: string = "order-confirmation";
    const response: GetEmailTemplateResponse1 = await getEmailTemplate(tenantId, templateId);
    const emailTemplate: CustomEmailTemplate | undefined = response.customEmailTemplate;
    const configParams: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]

---