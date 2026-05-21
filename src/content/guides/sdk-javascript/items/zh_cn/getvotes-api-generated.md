## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 响应

返回：[`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 示例

[inline-code-attrs-start title = 'getVotes 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42c-eu';
const urlId: string = 'article-7f9b';
const includeMetadata: boolean | undefined = true;
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]

---