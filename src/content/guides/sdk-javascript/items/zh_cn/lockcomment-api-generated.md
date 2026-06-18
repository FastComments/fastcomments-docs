## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## 示例

[inline-code-attrs-start title = 'lockComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f6c1';
const commentId: string = 'cmt-9a8b7c';
const broadcastId: string = 'brd_2026_06_15';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTY1MDAwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const lockedWithSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
const lockedWithoutSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---