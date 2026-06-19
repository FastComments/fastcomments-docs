## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updatableCommentParams | UpdatableCommentParams | 예 |  |
| contextUserId | string | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| isLive | boolean | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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