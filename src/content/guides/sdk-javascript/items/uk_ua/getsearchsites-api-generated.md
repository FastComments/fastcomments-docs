## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| value | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSites'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = 'fastcomments.com';
const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const responseWithSSO: ModerationSiteSearchResponse = await getSearchSites(searchValue, ssoToken);
const responseWithoutSSO: ModerationSiteSearchResponse = await getSearchSites('news.fastcompany.com');
[inline-code-end]

---