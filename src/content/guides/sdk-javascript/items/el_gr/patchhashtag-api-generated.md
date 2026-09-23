## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| tag | string | Ναι |  |
| updateHashTagBody | UpdateHashTagBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Κλήση χωρίς προαιρετικό σώμα
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Προετοιμασία σώματος για ενημέρωση
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]