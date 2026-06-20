## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSearchUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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