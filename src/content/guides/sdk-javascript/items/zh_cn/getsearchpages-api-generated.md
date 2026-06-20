## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchPages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = "homepage-recent-threads";
const ssoToken: string = "sso_user_7f9b2c3d";
const resultWithBoth: ModerationPageSearchResponse = await getSearchPages(searchValue, ssoToken);
const resultWithValueOnly: ModerationPageSearchResponse = await getSearchPages(searchValue);
const resultWithSSOOnly: ModerationPageSearchResponse = await getSearchPages(undefined, ssoToken);
[inline-code-end]

---