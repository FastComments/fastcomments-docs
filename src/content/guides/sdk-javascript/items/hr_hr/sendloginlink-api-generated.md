## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| redirectURL | string | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  // polja statusa
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