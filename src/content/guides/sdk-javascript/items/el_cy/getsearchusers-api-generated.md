## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| value | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSearchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const searchValue: string = 'jane.doe@acme-corp.com';
  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const responseWithSso: ModerationUserSearchResponse = await getSearchUsers(searchValue, ssoToken);
  const searchValue2: string = 'michael.brown';
  const responseWithoutSso: ModerationUserSearchResponse = await getSearchUsers(searchValue2);
  console.log(responseWithSso, responseWithoutSso);
})();
[inline-code-end]

---