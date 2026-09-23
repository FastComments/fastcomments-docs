## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| value | string | Ne |  |
| sso | string | Ne |  |

## Response

Vraća: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## Example

[inline-code-attrs-start title = 'getSearchPages Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const searchValue: string = "spam";
const ssoToken: string = "sso-abc123def456";

const result1: ModerationPageSearchResponse = await getSearchPages(tenantId);
const result2: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue);
const result3: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue, ssoToken);
[inline-code-end]