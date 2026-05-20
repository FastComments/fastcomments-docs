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
const tenantId: string = "acme_enterprises_01";
const createCommentParams: CreateCommentParams[] = [
  {
    content: "Thanks — this patch fixes the memory leak observed in staging.",
    authorId: "dev_482",
    postId: "issue_904",
    mentions: [{ userId: "lead_12", start: 8, end: 17 } as CommentUserMentionInfo],
    hashtags: [{ tag: "bugfix", indices: [60, 66] } as CommentUserHashTagInfo],
    createdAt: "2026-05-20T10:15:00Z"
  } as CreateCommentParams
];
const result: Array<SaveComment200Response> = await saveCommentsBulk(
  tenantId,
  createCommentParams,
  true,   // isLive
  true,   // doSpamCheck
  false,  // sendEmails
  true    // populateNotifications
);
[inline-code-end]
