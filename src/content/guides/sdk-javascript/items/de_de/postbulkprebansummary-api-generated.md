## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | Ja |  |
| includeByUserIdAndEmail | boolean | Nein |  |
| includeByIP | boolean | Nein |  |
| includeByEmailDomain | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkPreBanSummary.ts)

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const bulkParams: BulkPreBanParams = {
  accounts: [
    { userId: 'u_8729', email: 'bot123@malicious.com', ip: '192.0.2.55' },
    { userId: 'u_9901', email: 'spam.sender@cheapmeds.co', ip: '198.51.100.12' }
  ],
  reason: 'Automated pre-ban candidate import'
};
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const sso: string = 'sso_58fd3b2c-token';
const result: BulkPreBanSummary = await postBulkPreBanSummary(bulkParams, includeByUserIdAndEmail, includeByIP, includeByEmailDomain, sso);
[inline-code-end]

---