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
(async () => {
  const tenantId: string = 'tenant_7f4b21';
  const createCommentParams: CreateCommentParams[] = [
    {
      content: 'I learned a lot from this post. Thanks for the practical tips!',
      authorName: 'Sofia Ramos',
      authorEmail: 'sofia.ramos@techblog.com',
      userId: 'user_89237',
      threadId: 'article-2025-11-typescript-tips',
      url: 'https://techblog.com/posts/typescript-tips',
      mentions: [{ userId: 'user_102', username: 'devMike' } as CommentUserMentionInfo],
      hashtags: [{ tag: 'typescript' } as CommentUserHashTagInfo],
      createdAt: new Date().toISOString()
    }
  ];
  const isLive: boolean = true;
  const doSpamCheck: boolean = true;
  const sendEmails: boolean = false;
  const populateNotifications: boolean = true;
  const result: Array<SaveComment200Response> = await saveCommentsBulk(
    tenantId,
    createCommentParams,
    isLive,
    doSpamCheck,
    sendEmails,
    populateNotifications
  );
  console.log(result);
})();
[inline-code-end]
