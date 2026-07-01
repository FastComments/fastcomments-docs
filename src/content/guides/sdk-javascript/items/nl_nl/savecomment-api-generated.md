## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createCommentParams | CreateCommentParams | Ja |  |
| isLive | boolean | Nee |  |
| doSpamCheck | boolean | Nee |  |
| sendEmails | boolean | Nee |  |
| populateNotifications | boolean | Nee |  |

## Respons

Retourneert: [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'saveComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitComment() {
  const tenantId: string = "tenant_9f8e7d6c";
  const commentParams: CreateCommentParams = {
    text: "Great post, thanks for sharing!",
    authorId: "user_123abc",
    mentions: [] as CommentUserMentionInfo[],
    hashtags: [] as CommentUserHashTagInfo[]
  };
  const response: SaveCommentResponse = await saveComment(
    tenantId,
    commentParams,
    true,   // isLive
    false   // doSpamCheck
  );
  console.log(response);
}
submitComment();
[inline-code-end]