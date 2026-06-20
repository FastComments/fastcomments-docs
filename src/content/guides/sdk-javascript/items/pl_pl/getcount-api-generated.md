## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| textSearch | string | Nie |  |
| byIPFromComment | string | Nie |  |
| filter | string | Nie |  |
| searchFilters | string | Nie |  |
| demo | boolean | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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