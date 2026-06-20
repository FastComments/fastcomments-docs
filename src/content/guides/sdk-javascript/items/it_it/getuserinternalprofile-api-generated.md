## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| commentId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserInternalProfile'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const profileByCommentId: GetUserInternalProfileResponse = await getUserInternalProfile('comment_5f1e8a3b9c2d4');
const profileBySSOToken: GetUserInternalProfileResponse = await getUserInternalProfile(undefined, 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummypayload.signature');
[inline-code-end]

---