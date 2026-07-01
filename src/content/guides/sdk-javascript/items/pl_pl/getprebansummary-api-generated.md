## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| commentId | string | Tak |  |
| includeByUserIdAndEmail | boolean | Nie |  |
| includeByIP | boolean | Nie |  |
| includeByEmailDomain | boolean | Nie |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPreBanSummaryResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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