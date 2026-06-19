## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tag | string | Da |  |
| tenantId | string | Ne |  |
| updateHashTagBody | UpdateHashTagBody | Ne |  |

## Odgovor

Vraća: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Primjer

[inline-code-attrs-start title = 'patchHashTag Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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