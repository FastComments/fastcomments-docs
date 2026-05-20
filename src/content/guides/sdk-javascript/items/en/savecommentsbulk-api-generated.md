## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | Array<CreateCommentParams> | Yes |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Response

Returns: `Array<SaveComment200Response`

## Example

[inline-code-attrs-start title = 'saveCommentsBulk Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const createParams: CreateCommentParams[] = [
  {
    content: "Finished the draft for the Q2 report, please review.",
    authorId: "user_789",
    mentions: [{ userId: "user_456", startIndex: 34, length: 4 } as CommentUserMentionInfo],
    hashtags: [{ tag: "Q2", indices: [28, 30] } as CommentUserHashTagInfo]
  } as CreateCommentParams
];
const saveResult: SaveComment200Response[] = await saveCommentsBulk(
  tenantId,
  createParams,
  true,   // isLive
  true,   // doSpamCheck
  false,  // sendEmails
  true    // populateNotifications
);
[inline-code-end]
