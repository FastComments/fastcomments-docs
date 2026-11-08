## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| userId | string | नहीं |  |
| anonUserId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'unFlagComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt-20230915-001";
  const userId: string = "user-42";

  const unflaggedByUser: FlagCommentResponse = await unFlagComment(tenantId, commentId, userId);
  const unflaggedByAnon: FlagCommentResponse = await unFlagComment(tenantId, commentId, undefined, "anon-xyz");

  console.log(unflaggedByUser, unflaggedByAnon);
}
[inline-code-end]