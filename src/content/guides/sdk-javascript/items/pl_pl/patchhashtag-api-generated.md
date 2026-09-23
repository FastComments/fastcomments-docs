## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Odpowiedź

Zwraca: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Przykład

[inline-code-attrs-start title = 'patchHashTag Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Wywołanie bez opcjonalnego ciała
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Przygotuj ciało do aktualizacji
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]