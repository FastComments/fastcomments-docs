## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| namespace | string | Yes |  |
| component | string | Yes |  |
| locale | string | No |  |
| useFullTranslationIds | boolean | No |  |

## Réponse

Retourne : [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]