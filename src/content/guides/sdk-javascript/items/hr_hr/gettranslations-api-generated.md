## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| namespace | string | Da |  |
| component | string | Da |  |
| locale | string | Ne |  |
| useFullTranslationIds | boolean | Ne |  |

## Odgovor

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]

---