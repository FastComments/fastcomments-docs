## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Retourne : [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchConfigs() {
  const tenantId: string = "tenant_12345";

  const firstPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const secondPage: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 20);
}
[inline-code-end]

---