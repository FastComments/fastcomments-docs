## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSearchSites 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = 'fastcomments.com';
const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const responseWithSSO: ModerationSiteSearchResponse = await getSearchSites(searchValue, ssoToken);
const responseWithoutSSO: ModerationSiteSearchResponse = await getSearchSites('news.fastcompany.com');
[inline-code-end]