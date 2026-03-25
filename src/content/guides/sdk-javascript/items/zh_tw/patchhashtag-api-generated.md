## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 回應

回傳：[`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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