## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSearchPages 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = "homepage-recent-threads";
const ssoToken: string = "sso_user_7f9b2c3d";
const resultWithBoth: ModerationPageSearchResponse = await getSearchPages(searchValue, ssoToken);
const resultWithValueOnly: ModerationPageSearchResponse = await getSearchPages(searchValue);
const resultWithSSOOnly: ModerationPageSearchResponse = await getSearchPages(undefined, ssoToken);
[inline-code-end]

---