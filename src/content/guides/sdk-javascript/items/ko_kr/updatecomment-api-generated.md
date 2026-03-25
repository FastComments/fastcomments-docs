## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updatableCommentParams | UpdatableCommentParams | 예 |  |
| contextUserId | string | 아니요 |  |
| doSpamCheck | boolean | 아니요 |  |
| isLive | boolean | 아니요 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const id: string = "comment_20260325_4592";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Updated the response to include a link to the RFC and fixed a typo in the second paragraph.",
  editedByUserId: "user_8721",
  isVisible: true
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]