## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updatableCommentParams | UpdatableCommentParams | はい |  |
| contextUserId | string | いいえ |  |
| doSpamCheck | boolean | いいえ |  |
| isLive | boolean | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'updateComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-prod-01';
const id: string = 'cmt-000127';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Thanks — I updated the steps to include the missing config flag.',
  isHidden: false
};
const contextUserId: string = 'moderator_77';
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: APIEmptyResponse = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---