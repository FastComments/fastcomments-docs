## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Geeft terug: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-publishing-42";
const userIdOptional: string | undefined = "user_9d7b4c"; // kan in sommige flows undefined zijn (optioneel)
const id: string = userIdOptional ?? "user_default_0001";
const result: GetUser200Response = await getUser(tenantId, id);
console.log(result);
[inline-code-end]

---