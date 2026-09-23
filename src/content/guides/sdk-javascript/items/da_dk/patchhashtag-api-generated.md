## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| tag | string | Ja |  |
| updateHashTagBody | UpdateHashTagBody | Nej |  |

## Svar

Returnerer: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'patchHashTag Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Kald uden valgfri body
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Forbered body til opdatering
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]