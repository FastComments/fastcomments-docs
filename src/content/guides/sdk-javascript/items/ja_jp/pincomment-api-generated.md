## パラメータ

| 名称 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'pinComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82b1f9';
const commentId: string = 'cmt_9f8e7d6a';
const broadcastId: string = 'live_brdcst_2026_06_19';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const responseWithoutSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---