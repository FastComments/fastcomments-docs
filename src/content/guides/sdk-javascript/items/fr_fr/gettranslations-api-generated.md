## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| namespace | string | Oui |  |
| component | string | Oui |  |
| locale | string | Non |  |
| useFullTranslationIds | boolean | Non |  |

## Réponse

Returns: [`GetTranslationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getTranslations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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