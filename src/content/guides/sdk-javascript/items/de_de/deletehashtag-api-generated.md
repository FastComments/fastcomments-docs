## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tag | string | Ja |  |
| tenantId | string | Nein |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nein |  |

## Antwort

Rückgabe: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteHashTag Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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