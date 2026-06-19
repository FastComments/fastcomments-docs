---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| isFlagged | boolean | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'flagCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-inc-84";
const commentId: string = "b7f3e9a1-4c2d-4f6b-9f2a-123456789abc";
const isFlaggedOn: boolean = true;
const isFlaggedOff: boolean = false;
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlaggedOn);
const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlaggedOff, ssoToken);
[inline-code-end]

---