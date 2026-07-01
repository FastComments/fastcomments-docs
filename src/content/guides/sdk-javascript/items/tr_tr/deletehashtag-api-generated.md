## Parameters

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tag | string | Evet |  |
| tenantId | string | Hayır |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Hayır |  |

## Response

Döndürür: [`DeleteHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteHashTagResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteHashTag Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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