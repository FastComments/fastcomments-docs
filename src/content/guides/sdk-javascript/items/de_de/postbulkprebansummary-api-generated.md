## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | boolean | Nein |  |
| includeByIP | boolean | Nein |  |
| includeByEmailDomain | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Returns: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkPreBanSummary.ts)

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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