## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tag | string | Ja |  |
| tenantId | string | Nee |  |
| updateHashTagBody | UpdateHashTagBody | Nee |  |

## Response

Retourneert: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'patchHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "feature-ux-refresh";
const tenantId: string = "tenant_4f92c1";
const updateHashTagBody: UpdateHashTagBody = {
  label: "UX Refresh",
  description: "Track comments related to the 2026 UX redesign",
  isActive: true,
  metadata: { owner: "product-design", rolloutPhase: "phase-2" }
};
const response: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---