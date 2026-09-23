## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Odgovor

Vraća: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getVotesForUser Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "post_9876";
  const userId: string = "user_abcde";
  const anonUserId: string = "anon_5678";

  const responseWithUser: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
  const responseWithAnon: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
}

demo();
[inline-code-end]

---