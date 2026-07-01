## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| bulkPreBanParams | BulkPreBanParams | Tak |  |
| includeByUserIdAndEmail | boolean | Nie |  |
| includeByIP | boolean | Nie |  |
| includeByEmailDomain | boolean | Nie |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`PostBulkPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBulkPreBanSummaryResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład postBulkPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const bulkPreBanParams: BulkPreBanParams = {
    userIds: [10234, 56789],
    emails: ["spam_user@example.com", "abuse@badsite.org"],
    ips: ["203.0.113.45", "198.51.100.22"],
    emailDomains: ["maliciousdomain.com"]
  };

  const includeByUserIdAndEmail: boolean = true;
  const includeByIP: boolean = false;
  const includeByEmailDomain: boolean = true;
  const tenantId: string = "tenant_8f4b2c1a";
  const sso: string = "sso-3948abf0";

  const summary: PostBulkPreBanSummaryResponse = await postBulkPreBanSummary(
    bulkPreBanParams,
    includeByUserIdAndEmail,
    includeByIP,
    includeByEmailDomain,
    tenantId,
    sso
  );

  console.log(summary);
}

runExample();
[inline-code-end]