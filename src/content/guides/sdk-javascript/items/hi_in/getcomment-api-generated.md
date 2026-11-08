## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComment(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  const response: APIGetCommentResponse = await getComment(tenantId, commentId);
  const comment: APIComment | undefined = response.comment;

  // वैकल्पिक फ़ील्ड का प्रदर्शन
  const badgeInfo: CommentUserBadgeInfo | undefined = comment?.user?.badge;
  const hashtags: CommentUserHashTagInfo[] | undefined = comment?.user?.hashtags;
  const mentions: CommentUserMentionInfo[] | undefined = comment?.user?.mentions;
  const meta: APICommentBaseMeta | undefined = comment?.meta;

  console.log(comment?.id, badgeInfo?.type);
}

fetchComment();
[inline-code-end]