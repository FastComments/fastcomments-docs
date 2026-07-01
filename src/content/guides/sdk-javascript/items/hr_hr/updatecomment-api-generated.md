## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updatableCommentParams | UpdatableCommentParams | Da |  |
| contextUserId | string | Ne |  |
| doSpamCheck | boolean | Ne |  |
| isLive | boolean | Ne |  |

## Odgovor

Vraća: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // primjerni fieldovi; stvarni oblik ovisi o definiciji API-ja
  // npr., body: "Uređeni sadržaj komentara",
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