## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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