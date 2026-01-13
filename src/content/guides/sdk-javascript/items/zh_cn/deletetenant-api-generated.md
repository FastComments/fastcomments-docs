## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| sure | string | 否 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteTenant 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7b3f1a9e";
const id: string = "flag_9c4d2b1a";
const sure: string = "yes_confirm_delete";

const result: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
console.log(result);
[inline-code-end]

---