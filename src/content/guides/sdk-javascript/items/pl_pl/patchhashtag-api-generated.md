## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tag | string | Tak |  |
| tenantId | string | Nie |  |
| updateHashTagBody | UpdateHashTagBody | Nie |  |

## Odpowiedź

Zwraca: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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