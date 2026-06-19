## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tag | string | Ja |  |
| tenantId | string | Nee |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nee |  |

## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_7b2f9c";
const deleteHashTagRequestBody: DeleteHashTagRequestBody = { removedBy: "ops@acme-corp.com", force: true };
const result: APIEmptyResponse = await deleteHashTag(tag, tenantId, deleteHashTagRequestBody);
[inline-code-end]

---