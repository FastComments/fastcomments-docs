## פרמטרים

| שם | Type | נדרש | תיאור |
|------|------|----------|-------------|
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filter | string | לא |  |
| searchFilters | string | לא |  |
| demo | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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