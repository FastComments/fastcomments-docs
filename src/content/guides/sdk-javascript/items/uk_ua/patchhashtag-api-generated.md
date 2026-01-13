---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Так |  |
| tenantId | string | Ні |  |
| updateHashTagBody | UpdateHashTagBody | Ні |  |

## Відповідь

Повертає: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Приклад

[inline-code-attrs-start title = 'patchHashTag Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = 'release-2026';
const tenantId: string = 'tenant_42';
const updateHashTagBody: UpdateHashTagBody = {
  displayName: 'Release 2026',
  description: 'Discussions and notes for the 2026 product release',
  isActive: true
};
const result: PatchHashTag200Response = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---