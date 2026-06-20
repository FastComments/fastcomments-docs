---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| textSearch | string | Hayır |  |
| byIPFromComment | string | Hayır |  |
| filter | string | Hayır |  |
| searchFilters | string | Hayır |  |
| demo | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCount Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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