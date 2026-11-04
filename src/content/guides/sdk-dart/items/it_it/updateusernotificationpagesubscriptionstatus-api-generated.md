Abilita o disabilita le notifiche per una pagina. Quando gli utenti sono iscritti a una pagina, le notifiche vengono create
per i nuovi commenti principali e anche

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |
| url | string | query | Sì |  |
| pageTitle | string | query | Sì |  |
| subscribedOrUnsubscribed | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `UpdateUserNotificationPageSubscriptionStatusResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserNotificationPageSubscriptionStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final url = url_example; // String | 
final pageTitle = pageTitle_example; // String | 
final subscribedOrUnsubscribed = subscribedOrUnsubscribed_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: $e\n');
}
[inline-code-end]