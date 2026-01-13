## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## 示例

[inline-code-attrs-start title = 'getModerator 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_eu_4f8d2b9e";
const maybeModeratorId: string | undefined = "mod_91c3b7a2"; // optional source (could be undefined)
const moderator: GetModerator200Response = await getModerator(tenantId, maybeModeratorId!);
[inline-code-end]

---