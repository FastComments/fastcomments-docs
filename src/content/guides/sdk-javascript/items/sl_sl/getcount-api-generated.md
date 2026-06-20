## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| textSearch | string | Ne |  |
| byIPFromComment | string | Ne |  |
| filter | string | Ne |  |
| searchFilters | string | Ne |  |
| demo | boolean | Ne |  |
| sso | string | Ne |  |

## Odziv

Vrne: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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