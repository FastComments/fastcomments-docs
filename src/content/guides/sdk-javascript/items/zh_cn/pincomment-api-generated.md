## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'pinComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82b1f9';
const commentId: string = 'cmt_9f8e7d6a';
const broadcastId: string = 'live_brdcst_2026_06_19';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const responseWithoutSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---