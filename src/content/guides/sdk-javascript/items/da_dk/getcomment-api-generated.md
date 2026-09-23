## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComment(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  const response: APIGetCommentResponse = await getComment(tenantId, commentId);
  const comment: APIComment | undefined = response.comment;

  // Demonstration af valgfrie felter
  const badgeInfo: CommentUserBadgeInfo | undefined = comment?.user?.badge;
  const hashtags: CommentUserHashTagInfo[] | undefined = comment?.user?.hashtags;
  const mentions: CommentUserMentionInfo[] | undefined = comment?.user?.mentions;
  const meta: APICommentBaseMeta | undefined = comment?.meta;

  console.log(comment?.id, badgeInfo?.type);
}

fetchComment();
[inline-code-end]

---