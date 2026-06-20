## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filter | string | いいえ |  |
| searchFilters | string | いいえ |  |
| demo | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## 例

[inline-code-attrs-start title = 'getCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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