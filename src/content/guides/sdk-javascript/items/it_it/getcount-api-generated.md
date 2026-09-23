## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filter | string | No |  |
| searchFilters | string | No |  |
| demo | boolean | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const basicCount: ModerationAPICountCommentsResponse = await getCount(tenantId);

const detailedCount: ModerationAPICountCommentsResponse = await getCount(
  tenantId,
  "spam",
  "192.168.1.100",
  "status:pending",
  "user:john",
  true,
  "sso_token_abc"
);
[inline-code-end]