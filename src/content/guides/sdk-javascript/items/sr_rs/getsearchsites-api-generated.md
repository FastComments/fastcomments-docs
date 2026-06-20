## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| value | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getSearchSites'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = 'fastcomments.com';
const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const responseWithSSO: ModerationSiteSearchResponse = await getSearchSites(searchValue, ssoToken);
const responseWithoutSSO: ModerationSiteSearchResponse = await getSearchSites('news.fastcompany.com');
[inline-code-end]

---