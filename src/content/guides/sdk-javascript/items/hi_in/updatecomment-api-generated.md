## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updatableCommentParams | UpdatableCommentParams | हाँ |  |
| contextUserId | string | नहीं |  |
| doSpamCheck | boolean | नहीं |  |
| isLive | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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