## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Hayır |  |
| urlId | string | Hayır |  |
| fromCommentId | string | Hayır |  |
| viewed | boolean | Hayır |  |
| type | string | Hayır |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getNotifications Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "tenant_42";
  const userId: string = "user_1001";
  const urlId: string = "url_2023";
  const viewed: boolean = true;
  const skip: number = 0;

  const notifications: GetNotificationsResponse = await getNotifications(
    tenantId,
    userId,
    urlId,
    undefined,
    viewed,
    undefined,
    skip
  );

  console.log(notifications);
}
[inline-code-end]