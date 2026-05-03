## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| namespace | string | 예 |  |
| component | string | 예 |  |
| locale | string | 아니요 |  |
| useFullTranslationIds | boolean | 아니요 |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsBase: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread");
const translationsSpanishFullIds: GetTranslationsResponse = await getTranslations("acme-site-482", "commentThread", "es-ES", true);
[inline-code-end]

---