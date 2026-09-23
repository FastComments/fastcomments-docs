## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| spam | boolean | 아니오 |  |
| permNotSpam | boolean | 아니오 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'postSetCommentSpamStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---