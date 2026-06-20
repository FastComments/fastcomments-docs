## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filter | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| demo | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const countResult: ModerationAPICountCommentsResponse = await getCount(
    "reported harassment",
    "203.0.113.7",
    "status:pending",
    undefined,
    false,
    "sso_user_789.jwt.token"
  );
  console.log(countResult);
})();
[inline-code-end]

---