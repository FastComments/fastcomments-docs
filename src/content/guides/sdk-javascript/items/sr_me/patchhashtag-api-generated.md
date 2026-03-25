## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tag | string | Да |  |
| tenantId | string | Не |  |
| updateHashTagBody | UpdateHashTagBody | Не |  |

## Одговор

Враћа: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Пример

[inline-code-attrs-start title = 'patchHashTag Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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