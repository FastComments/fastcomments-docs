## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| namespace | string | Evet |  |
| component | string | Evet |  |
| locale | string | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]

---