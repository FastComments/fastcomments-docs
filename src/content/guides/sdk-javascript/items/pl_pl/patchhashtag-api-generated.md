## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tag | string | Tak |  |
| tenantId | string | Nie |  |
| updateHashTagBody | UpdateHashTagBody | Nie |  |

## Odpowiedź

Zwraca: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTag200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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