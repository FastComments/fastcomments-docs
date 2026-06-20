## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| urlId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'putReopenThread Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const reopenResultWithSso: APIEmptyResponse = await putReopenThread("th_3c9b2a7f", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEyM30.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c");
const reopenResultNoSso: APIEmptyResponse = await putReopenThread("th_7a4e5c1d");
[inline-code-end]