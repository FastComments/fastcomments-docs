## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

返回：[`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getEmailTemplates 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
  const tenantId: string = "acme-corp-123";
  const skip: number = 20;
  const templatesWithSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId, skip);
  const templatesWithoutSkip: GetEmailTemplatesResponse = await getEmailTemplates(tenantId);
}
[inline-code-end]

---