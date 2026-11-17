## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createCommentParams | CreateCommentParams | हाँ |  |
| isLive | boolean | नहीं |  |
| doSpamCheck | boolean | नहीं |  |
| sendEmails | boolean | नहीं |  |
| populateNotifications | boolean | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'saveComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'ABC_5a1f8f2e3d4c5b6a7d8e9f0a';

const createCommentParams: CreateCommentParams = {
  url: 'https://example-blog.com/posts/fastcomments-review',
  comment: 'This article really helped me understand performance tuning. Thanks!',
  handle: 'jane_doe_dev',
  email: 'jane.doe@example.com',
  parentId: '123e4567-e89b-12d3-a456-426614174000',
  mentions: [
    {
      handle: 'author_bob',
      email: 'bob@fastcomments.com'
    }
  ]
};

const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = true;

const response: APISaveCommentResponse = await saveComment(
  tenantId,
  createCommentParams,
  isLive,
  doSpamCheck,
  sendEmails,
  true
);

const commentStatus: APIStatus | undefined = response.status;
[inline-code-end]