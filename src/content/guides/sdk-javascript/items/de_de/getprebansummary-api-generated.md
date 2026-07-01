## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| commentId | string | Ja |  |
| includeByUserIdAndEmail | boolean | Nein |  |
| includeByIP | boolean | Nein |  |
| includeByEmailDomain | boolean | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`GetPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPreBanSummaryResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getPreBanSummary Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "5f8d0c2e4b1a2c3d4e5f6a7b";
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const tenantId: string = "tenant-001";
const sso: string = "sso-xyz-123";

const preBanSummary: GetPreBanSummaryResponse = await getPreBanSummary(
  commentId,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  tenantId,
  sso
);
[inline-code-end]