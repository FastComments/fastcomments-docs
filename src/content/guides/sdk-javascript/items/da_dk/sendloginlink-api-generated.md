## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| redirectURL | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'sendLoginLink Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  // statusfelter
}

interface APIEmptyResponse {
  status: APIStatus;
}

(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-98765";
  const redirectURL: string = "https://app.example.com/welcome";

  const responseWithRedirect: APIEmptyResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const responseWithoutRedirect: APIEmptyResponse = await sendLoginLink(tenantId, userId);
})();
[inline-code-end]