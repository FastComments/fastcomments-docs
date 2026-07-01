## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| includeByUserIdAndEmail | boolean | No |  |
| includeByIP | boolean | No |  |
| includeByEmailDomain | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απάντηση

Επιστρέφει: [`GetPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPreBanSummaryResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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