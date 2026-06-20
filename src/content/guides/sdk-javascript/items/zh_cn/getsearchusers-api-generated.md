## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const searchValue: string = 'jane.doe@acme-corp.com';
  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const responseWithSso: ModerationUserSearchResponse = await getSearchUsers(searchValue, ssoToken);
  const searchValue2: string = 'michael.brown';
  const responseWithoutSso: ModerationUserSearchResponse = await getSearchUsers(searchValue2);
  console.log(responseWithSso, responseWithoutSso);
})();
[inline-code-end]

---