## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updatableCommentParams | UpdatableCommentParams | Ja |  |
| contextUserId | string | Nein |  |
| doSpamCheck | boolean | Nein |  |
| isLive | boolean | Nein |  |

## Antwort

Rückgabe: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // Beispiel-Felder; tatsächliche Struktur hängt von der API-Definition ab
  // z. B., body: "Edited comment content",
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

---