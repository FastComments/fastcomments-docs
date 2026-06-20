## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| textSearch | string | Nee |  |
| byIPFromComment | string | Nee |  |
| filter | string | Nee |  |
| searchFilters | string | Nee |  |
| demo | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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