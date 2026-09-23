## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| tag | string | Sì |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Risposta

Restituisce: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Chiamata senza corpo opzionale
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Preparare il corpo per l'aggiornamento
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]