## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filter | string | Non |  |
| searchFilters | string | Non |  |
| demo | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---