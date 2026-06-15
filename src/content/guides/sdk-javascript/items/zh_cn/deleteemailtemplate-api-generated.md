## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteEmailTemplate 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9c4f1b2a";
const id: string = "emailtmpl_4d2b9a5e";
const requestorNote: string | undefined = undefined; // 可选元数据（函数不需要）
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
[inline-code-end]