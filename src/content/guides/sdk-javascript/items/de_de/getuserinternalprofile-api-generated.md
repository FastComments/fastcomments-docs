## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserInternalProfile Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const profileByCommentId: GetUserInternalProfileResponse = await getUserInternalProfile('comment_5f1e8a3b9c2d4');
const profileBySSOToken: GetUserInternalProfileResponse = await getUserInternalProfile(undefined, 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummypayload.signature');
[inline-code-end]

---