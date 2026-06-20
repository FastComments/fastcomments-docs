## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filter | string | Nein |  |
| searchFilters | string | Nein |  |
| demo | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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