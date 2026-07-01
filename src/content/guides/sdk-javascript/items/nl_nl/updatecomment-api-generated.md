## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updatableCommentParams | UpdatableCommentParams | Ja |  |
| contextUserId | string | Nee |  |
| doSpamCheck | boolean | Nee |  |
| isLive | boolean | Nee |  |

## Respons

Retourneert: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // voorbeeldvelden; de werkelijke structuur is afhankelijk van de API-definitie
  // b.v., body: "Bewerkt commentaarinhoud",
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