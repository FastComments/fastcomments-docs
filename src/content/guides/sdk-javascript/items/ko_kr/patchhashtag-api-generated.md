## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | 예 |  |
| tenantId | string | 아니오 |  |
| updateHashTagBody | UpdateHashTagBody | 아니오 |  |

## 응답

반환: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## 예제

[inline-code-attrs-start title = 'patchHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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