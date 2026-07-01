## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`DeleteQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteQuestionConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteQuestionConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletion(): Promise<void> {
  const tenantId: string = "tenant_8f5a2c9d";
  const configId: string = "questionConfig_4b7e1f";
  const deleteResult: DeleteQuestionConfigResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(deleteResult);
}
void runDeletion();
[inline-code-end]

---