## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tag | string | Tak |  |
| tenantId | string | Nie |  |
| deleteHashTagRequest | DeleteHashTagRequest | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "spring-sale-2026";
const tenantId: string = "tenant-9876";
const deleteHashTagRequest: DeleteHashTagRequest = {
  requestedBy: "admin@retailco.com",
  reason: "Campaign ended; remove associated auto-tags",
  cascadeDelete: true
};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

---