## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filter | string | Non |  |
| searchFilters | string | Non |  |
| demo | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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