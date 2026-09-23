## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| spam | boolean | No |  |
| permNotSpam | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";

const spam: boolean = true;
const permNotSpam: boolean = false;
const broadcastId: string = "broadcast_2023";
const sso: string = "sso_user_5678";

const resultFull: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId,
  spam,
  permNotSpam,
  broadcastId,
  sso
);

const resultMinimal: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId
);
[inline-code-end]