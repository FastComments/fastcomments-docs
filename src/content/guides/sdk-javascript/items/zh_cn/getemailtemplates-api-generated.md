## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回： [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## 示例

[inline-code-attrs-start title = 'getEmailTemplates 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_5f3a9c2b';
  const templates: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const skip: number = 20;
  const pagedTemplates: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
  console.log(templates, pagedTemplates);
}
main();
[inline-code-end]

---