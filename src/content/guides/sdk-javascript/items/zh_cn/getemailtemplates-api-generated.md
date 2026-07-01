## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getEmailTemplates 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // 调用时不包含可选的 'skip'
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // 调用时包含可选的 'skip' 参数
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]