## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Przykład

[inline-code-attrs-start title = 'sendLoginLink Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  // pola statusu
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

---