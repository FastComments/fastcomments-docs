## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantSuffix: string | undefined = undefined;
const tenantId: string = tenantSuffix ? `tenant-${tenantSuffix}` : 'tenant-9142a7';
const id: string = '3f9b2a44-1c2e-4d3b-9f6a-8e7c6d5b2a1f';
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, id);
console.log(result);
[inline-code-end]

---