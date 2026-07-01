## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tag | string | Tak |  |
| tenantId | string | Nie |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Nie |  |

## Odpowiedź

Zwraca: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## Przykład

[inline-code-attrs-start title = 'deleteHashTag Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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