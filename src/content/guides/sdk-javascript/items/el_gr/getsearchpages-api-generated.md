## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| value | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSearchPages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const searchValue: string = "spam";
const ssoToken: string = "sso-abc123def456";

const result1: ModerationPageSearchResponse = await getSearchPages(tenantId);
const result2: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue);
const result3: ModerationPageSearchResponse = await getSearchPages(tenantId, searchValue, ssoToken);
[inline-code-end]