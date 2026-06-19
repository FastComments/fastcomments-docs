---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Antwort

Gibt zurück: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'patchHashTag Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_8421";
const updateHashTagBody: UpdateHashTagBody = {
  name: "Release Notes",
  description: "Thread for discussing feature releases and changelogs",
  isActive: true
};
const result: UpdateHashTagResponse = await patchHashTag(tag, tenantId, updateHashTagBody);
[inline-code-end]

---