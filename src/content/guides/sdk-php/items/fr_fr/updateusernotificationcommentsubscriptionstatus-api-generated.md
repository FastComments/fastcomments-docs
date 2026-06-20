Activer ou désactiver les notifications pour un commentaire spécifique.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| notificationId | string | path | Oui |  |
| optedInOrOut | string | path | Oui |  |
| commentId | string | query | Oui |  |
| sso | string | query | Non |  |

## Response

Renvoie : [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationCommentSubscriptionStatusResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple updateUserNotificationCommentSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, fournissez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // C'est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$notification_id = 'notification_id_example'; // string
$opted_in_or_out = 'opted_in_or_out_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateUserNotificationCommentSubscriptionStatus($tenant_id, $notification_id, $opted_in_or_out, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]