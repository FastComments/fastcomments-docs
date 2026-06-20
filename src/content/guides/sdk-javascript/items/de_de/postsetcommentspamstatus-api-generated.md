## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| spam | boolean | Nein |  |
| permNotSpam | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentSpamStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_9f8b3a2e';
const spam: boolean = false;
const permNotSpam: boolean = true;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signedToken';
const result: APIEmptyResponse = await postSetCommentSpamStatus(commentId, spam, permNotSpam, sso);
[inline-code-end]

---