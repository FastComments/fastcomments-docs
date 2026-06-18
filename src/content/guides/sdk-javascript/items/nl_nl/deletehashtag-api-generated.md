## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tag | string | Ja |  |
| tenantId | string | Nee |  |
| deleteHashTagRequest | DeleteHashTagRequest | Nee |  |

## Antwoord

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_72a1";
const deleteHashTagRequest: DeleteHashTagRequest = {
  reason: "consolidate-duplicates",
  requestedBy: "moderator@dailypress.com",
  forceDelete: true
};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

---