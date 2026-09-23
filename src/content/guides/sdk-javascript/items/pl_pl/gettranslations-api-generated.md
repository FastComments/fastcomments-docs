## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| namespace | string | Tak |  |
| component | string | Tak |  |
| locale | string | Nie |  |
| useFullTranslationIds | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getTranslations Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]

---