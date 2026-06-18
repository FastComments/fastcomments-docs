## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 回應

回傳: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## 範例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // 示範可選參數
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---