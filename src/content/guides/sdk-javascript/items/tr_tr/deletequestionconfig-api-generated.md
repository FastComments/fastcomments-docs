## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteQuestionConfig Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function executeDeletion() {
  const tenantId: string = "tenant_12345";
  const configId: string = "config_9876";
  const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(result);
}
executeDeletion();
[inline-code-end]

---