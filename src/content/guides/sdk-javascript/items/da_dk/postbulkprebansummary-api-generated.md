## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | boolean | Nej |  |
| includeByIP | boolean | Nej |  |
| includeByEmailDomain | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkPreBanSummary.ts)

## Eksempel

[inline-code-attrs-start title = 'postBulkPreBanSummary Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1f2e3d4-5678-90ab-cdef-1234567890ab";

const bulkPreBanParams: BulkPreBanParams = {
  userIds: ["user123"],
  emails: ["spam@example.com"],
  ips: ["192.0.2.1"],
  emailDomains: ["example.com"]
};

const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = false;
const includeByEmailDomain: boolean = true;
const sso: string = "sso-token-abc123";

const summary: BulkPreBanSummary = await postBulkPreBanSummary(
  tenantId,
  bulkPreBanParams,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  sso
);
[inline-code-end]