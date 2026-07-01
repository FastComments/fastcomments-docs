Habilitar o deshabilitar notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones para nuevos comentarios raíz, y también

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|--------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| url | string | Sí |  |
| pageTitle | string | Sí |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-2024";
  const urlId: string = "page-5678";
  const url: string = "https://example.com/articles/typescript-tips";
  const pageTitle: string = "Top TypeScript Tips";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso-token-xyz";

  const resultWithSso: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  const resultWithoutSso: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed
  );
})();
[inline-code-end]