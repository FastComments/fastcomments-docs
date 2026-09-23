## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | boolean | Nee |  |
| includeByIP | boolean | Nee |  |
| includeByEmailDomain | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkPreBanSummary.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postBulkPreBanSummary Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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