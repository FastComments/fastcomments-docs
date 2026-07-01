## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vrne: [`GetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUserBadge Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-001";
  const badgeId: string = "badge-5f9d3a2b";

  const badgeResponse: GetUserBadgeResponse = await getUserBadge(tenantId, badgeId);

  // Varno dostopajte do neobveznih polj
  const badgeName: string | undefined = badgeResponse.userBadge?.name;
  console.log(`Badge ID: ${badgeId}, Name: ${badgeName ?? "Unnamed"}`);
}

runExample();
[inline-code-end]

---