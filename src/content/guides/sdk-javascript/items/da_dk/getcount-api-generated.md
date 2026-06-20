## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| textSearch | string | Nej |  |
| byIPFromComment | string | Nej |  |
| filter | string | Nej |  |
| searchFilters | string | Nej |  |
| demo | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getCount-eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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