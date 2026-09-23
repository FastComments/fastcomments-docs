## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| tag | string | Ja |  |
| updateHashTagBody | UpdateHashTagBody | Nein |  |

## Response

Rückgabe: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'patchHashTag Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Aufruf ohne optionalen Body
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Body für das Update vorbereiten
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]