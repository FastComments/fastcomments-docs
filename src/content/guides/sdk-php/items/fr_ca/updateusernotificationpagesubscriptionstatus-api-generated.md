Activer ou désactiver les notifications pour une page. Lorsque les utilisateurs sont abonnés à une page, des notifications sont créées
pour les nouveaux commentaires racines, et aussi

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| url | string | query | Oui |  |
| pageTitle | string | query | Oui |  |
| subscribedOrUnsubscribed | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple updateUserNotificationPageSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$url = 'url_example'; // string
$page_title = 'page_title_example'; // string
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]