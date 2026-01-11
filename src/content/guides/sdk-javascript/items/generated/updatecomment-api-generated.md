## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7b6f3a2c";
const id: string = "cmt_5f4e3a2b1d";
const updatableCommentParams: UpdatableCommentParams = {
  content: "I've updated my comment to add more context and fix a typo.",
  authorName: "Jordan Lee",
  meta: { editedBy: "moderator_12", tags: ["clarified"] },
  isVisible: true
};
const contextUserId: string = "user_98c3b";
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]
