## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| namespace | string | 예 |  |
| component | string | 예 |  |
| locale | string | 아니요 |  |
| useFullTranslationIds | boolean | 아니요 |  |

## 응답

반환: [`GetTranslations200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslations200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const namespaceName: string = 'site-ui';
const componentName: string = 'comment-widget';
const locale: string = 'fr-FR';
const useFullTranslationIds: boolean = true;
const translationsWithLocale: GetTranslations200Response = await getTranslations(namespaceName, componentName, locale, useFullTranslationIds);
const translationsDefault: GetTranslations200Response = await getTranslations(namespaceName, componentName);
[inline-code-end]

---