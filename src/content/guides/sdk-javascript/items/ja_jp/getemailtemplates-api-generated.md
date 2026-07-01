## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## レスポンス

返り値: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## 例

[inline-code-attrs-start title = 'getEmailTemplates の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // オプションの 'skip' を省いて呼び出す
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // オプションの 'skip' パラメータを指定して呼び出す
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]

---