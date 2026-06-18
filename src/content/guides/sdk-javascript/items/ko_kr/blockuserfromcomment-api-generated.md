## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| blockFromCommentParams | BlockFromCommentParams | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'blockUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b4c";
const id: string = "comment_9a8b7c6d";
const blockFromCommentParams: BlockFromCommentParams = {
  reason: "Repeated spam links",
  durationHours: 168,
  notifyModerators: true
};
const userId: string | undefined = "user_42";
const anonUserId: string | undefined = undefined;
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId, anonUserId);
[inline-code-end]