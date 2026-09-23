Aktivieren oder deaktivieren Sie Benachrichtigungen für eine Seite. Wenn Benutzer eine Seite abonniert haben, werden Benachrichtigungen für neue Hauptkommentare erstellt, und außerdem

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Ja |  |
| pageTitle | string | Ja |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const url: string = "https://example.com/articles/awesome-article";
  const pageTitle: string = "Awesome Article";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso_token_abc";

  const response: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  console.log(response);
}

runExample();
[inline-code-end]