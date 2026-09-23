## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filter | string | Nein |  |
| searchFilters | string | Nein |  |
| demo | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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