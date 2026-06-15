## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| sure | string | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteTenant 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_742b9c';
const flagId: string = 'flag_1a2b3c';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId);
const sureConfirmation: string = 'confirmed';
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId, sureConfirmation);
[inline-code-end]

---