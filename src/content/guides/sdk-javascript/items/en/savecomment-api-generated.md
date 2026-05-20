## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | Yes |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Response

Returns: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## Example

[inline-code-attrs-start title = 'saveComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-17";
const createCommentParams: CreateCommentParams = {
  body: "Thanks — this clarified the deployment. Suggest adding a note about step 3.",
  authorId: "user_789",
  threadId: "article_342",
  clientCreatedAt: new Date().toISOString()
};
const saveResult: SaveComment200Response = await saveComment(tenantId, createCommentParams, true, false, true, true);
[inline-code-end]
