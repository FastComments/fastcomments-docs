## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| value | string | 任意 |  |
| sso | string | 任意 |  |

## レスポンス

戻り値: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## 例

[inline-code-attrs-start title = 'getSearchPages の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = "homepage-recent-threads";
const ssoToken: string = "sso_user_7f9b2c3d";
const resultWithBoth: ModerationPageSearchResponse = await getSearchPages(searchValue, ssoToken);
const resultWithValueOnly: ModerationPageSearchResponse = await getSearchPages(searchValue);
const resultWithSSOOnly: ModerationPageSearchResponse = await getSearchPages(undefined, ssoToken);
[inline-code-end]

---