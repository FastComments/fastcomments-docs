## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nee |  |
| useFullTranslationIds | boolean | Nee |  |

## Respons

Retourneert: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]