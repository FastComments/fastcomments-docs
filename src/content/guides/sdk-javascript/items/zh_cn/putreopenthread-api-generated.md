## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| urlId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'putReopenThread 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const reopenResultWithSso: APIEmptyResponse = await putReopenThread("th_3c9b2a7f", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEyM30.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c");
const reopenResultNoSso: APIEmptyResponse = await putReopenThread("th_7a4e5c1d");
[inline-code-end]

---