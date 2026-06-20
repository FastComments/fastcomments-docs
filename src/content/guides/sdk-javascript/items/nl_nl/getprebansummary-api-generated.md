## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| commentId | string | Ja |  |
| includeByUserIdAndEmail | boolean | Nee |  |
| includeByIP | boolean | Nee |  |
| includeByEmailDomain | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPreBanSummary Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt-9f7b2e3d-45a1';
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const summary: PreBanSummary = await getPreBanSummary(
  commentId,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  sso
);
[inline-code-end]

---