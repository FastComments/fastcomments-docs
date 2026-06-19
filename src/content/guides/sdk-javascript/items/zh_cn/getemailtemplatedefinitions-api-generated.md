## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 响应

返回：[`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// 可选参数（如果受支持）可以作为第二个参数传递，例如 getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---