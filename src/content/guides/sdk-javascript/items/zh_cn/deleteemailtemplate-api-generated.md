## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteEmailTemplate 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const templateId: string = 'tmpl_3fa85f64-5717-4562-b3fc-2c963f66afa6';
const optionalStatus: APIStatus | undefined = undefined;
const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---