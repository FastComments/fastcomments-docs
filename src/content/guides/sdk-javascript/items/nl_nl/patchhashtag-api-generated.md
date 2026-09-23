## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| tag | string | Ja |  |
| updateHashTagBody | UpdateHashTagBody | Nee |  |

## Respons

Retourneert: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'patchHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Aanroep zonder optionele body
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Voorbereiden body voor update
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]

---