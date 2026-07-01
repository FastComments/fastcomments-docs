## Parametre

| Navn | Type | Krævet | Beskrivelse |
|------|------|--------|-------------|
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | boolean | Nej |  |
| includeByIP | boolean | Nej |  |
| includeByEmailDomain | boolean | Nej |  |
| tenantId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PostBulkPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBulkPreBanSummaryResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postBulkPreBanSummary Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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