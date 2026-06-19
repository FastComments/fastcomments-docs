## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## 示例

[inline-code-attrs-start title = 'deleteV1PageReact 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const urlId: string = 'post-5f2a8b3c9d';
const deletedReaction: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
[inline-code-end]

---