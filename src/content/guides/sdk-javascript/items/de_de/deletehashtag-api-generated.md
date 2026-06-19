## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tag | string | Ja |  |
| tenantId | string | Nein |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteHashTag Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_7b2f9c";
const deleteHashTagRequestBody: DeleteHashTagRequestBody = { removedBy: "ops@acme-corp.com", force: true };
const result: APIEmptyResponse = await deleteHashTag(tag, tenantId, deleteHashTagRequestBody);
[inline-code-end]

---