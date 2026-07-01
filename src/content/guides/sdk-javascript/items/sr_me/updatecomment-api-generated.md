## Parameters

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Response

Returns: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Example

[inline-code-attrs-start title = 'updateComment primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // primjeri polja; stvarni oblik zavisi od definicije API‑ja
  // npr., body: "Edited comment content",
};

const contextUserId: string = "user_abcde";
const doSpamCheck: boolean = true;
const isLive: boolean = false;

const result: UpdateCommentResponse = await updateComment(
  tenantId,
  commentId,
  updatableCommentParams,
  contextUserId,
  doSpamCheck,
  isLive
);
[inline-code-end]