## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentsByIdsParams | CommentsByIdsParams | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'postCommentsByIds उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f8b7c6d-1234-5678-90ab-cdef12345678';
const commentsByIdsParams: CommentsByIdsParams = {
  commentIds: ['comment-1', 'comment-2'],
  includeUserBadges: true
};
const responseWithoutSso: ModerationAPIChildCommentsResponse = await postCommentsByIds(tenantId, commentsByIdsParams);
const sso: string = 'sso-abc123def456';
const responseWithSso: ModerationAPIChildCommentsResponse = await postCommentsByIds(tenantId, commentsByIdsParams, sso);
[inline-code-end]