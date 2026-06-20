## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| textSearch | string | Όχι |  |
| byIPFromComment | string | Όχι |  |
| filter | string | Όχι |  |
| searchFilters | string | Όχι |  |
| demo | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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