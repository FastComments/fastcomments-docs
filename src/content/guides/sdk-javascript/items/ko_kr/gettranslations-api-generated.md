## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| namespace | string | 예 |  |
| component | string | 예 |  |
| locale | string | 아니오 |  |
| useFullTranslationIds | boolean | 아니오 |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const translationsFull: GetTranslationsResponse = await getTranslations("site-comments", "comment-form", "fr-FR", true);
const translationsDefault: GetTranslationsResponse = await getTranslations("admin-dashboard", "notification-center");
[inline-code-end]

---