## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| sure | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_9876";

  const resultWithoutSure: APIEmptyResponse = await deleteTenant(tenantId, userId);
  const resultWithSure: APIEmptyResponse = await deleteTenant(tenantId, userId, "confirm");
}

main();
[inline-code-end]