## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| namespace | string | Tak |  |
| component | string | Tak |  |
| locale | string | Nie |  |
| useFullTranslationIds | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const namespace: string = 'blog';
  const component: string = 'comment-editor';
  const locale: string = 'fr-FR';
  const useFullTranslationIds: boolean = true;

  const basicTranslations: GetTranslationsResponse1 = await getTranslations(namespace, component);
  const fullTranslations: GetTranslationsResponse1 = await getTranslations(namespace, component, locale, useFullTranslationIds);
})();
[inline-code-end]