## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 响应

返回：[`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEmailTemplates() {
    const tenantId: string = "acme-corp-456";
    const result: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
    const status: APIStatus = result.status;
    const definitions: EmailTemplateDefinition[] = result.definitions ?? [];
    console.log(`Status: ${status.code}, Templates: ${definitions.length}`);
}
[inline-code-end]