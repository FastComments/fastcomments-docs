---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| commentId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserInternalProfile'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const profileByCommentId: GetUserInternalProfileResponse = await getUserInternalProfile('comment_5f1e8a3b9c2d4');
const profileBySSOToken: GetUserInternalProfileResponse = await getUserInternalProfile(undefined, 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummypayload.signature');
[inline-code-end]

---