## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | boolean | Yes |  |
| sso | string | No |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'flagCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";
const isFlagged: boolean = true;
const sso: string = "sso_user_5678";

const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, false);
[inline-code-end]