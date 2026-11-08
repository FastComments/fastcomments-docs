## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| userId | string | नहीं |  |
| anonUserId | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'flagComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const userId: string = "user-42";
  const anonUserId: string = "anon-abc123";

  const response: FlagCommentResponse = await flagComment(tenantId, commentId, userId, anonUserId);
  console.log(response);
})();
[inline-code-end]