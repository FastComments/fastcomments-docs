## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tag | string | Ja |  |
| tenantId | string | Nee |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nee |  |

## Respons

Retourneert: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteHashTag Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tag: string = "announcement";
  const tenantId: string = "tenant_9876";
  const requestBody: DeleteHashTagRequestBody = {
    confirmDeletion: true
  };
  const response: DeleteHashTagResponse = await deleteHashTag(tag, tenantId, requestBody);
  console.log(response);
})();
[inline-code-end]

---