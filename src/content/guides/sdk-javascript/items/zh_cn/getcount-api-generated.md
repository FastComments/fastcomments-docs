---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filter | string | 否 |  |
| searchFilters | string | 否 |  |
| demo | boolean | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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