## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| search | string | Oui |  |
| locale | string | Non |  |
| rating | string | Non |  |
| page | number | Non |  |

## Réponse

Retourne : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const search: string = "celebration fireworks";
  const locale: string = "fr-FR";
  const rating: string = "g";
  const page: number = 1;

  const fullResult: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  const minimalResult: GetGifsSearchResponse = await getGifsSearch(tenantId, "dog memes");
}

runExample();
[inline-code-end]