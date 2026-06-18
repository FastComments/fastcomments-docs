## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tag | string | Ja |  |
| tenantId | string | Nee |  |
| updateHashTagBody | UpdateHashTagBody | Nee |  |

## Respons

Retourneert: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'patchHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "feature-request";
const tenantId: string = "tenant_8f7a3b2c";
const updateHashTagBody: UpdateHashTagBody = {
  displayName: "Feature Request",
  description: "Use this tag for requests to add new features to the product",
  enabled: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---