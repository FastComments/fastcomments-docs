## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updatableCommentParams | UpdatableCommentParams | Evet |  |
| contextUserId | string | Hayır |  |
| doSpamCheck | boolean | Hayır |  |
| isLive | boolean | Hayır |  |

## Response

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'updateComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "comment-9876543210";
const updateParams: UpdatableCommentParams = {
  content: "Edited comment content",
  isApproved: true
};

const contextUserId: string = "user-11223344";
const doSpamCheck: boolean = true;
const isLive: boolean = false;

const response: APIEmptyResponse = await updateComment(
  tenantId,
  commentId,
  updateParams,
  contextUserId,
  doSpamCheck,
  isLive
);
[inline-code-end]