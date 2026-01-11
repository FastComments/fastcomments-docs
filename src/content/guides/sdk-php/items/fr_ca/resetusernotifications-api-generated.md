## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| afterId | string | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotifications200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // chaîne
$after_id = 'after_id_example'; // chaîne
$after_created_at = 56; // entier
$unread_only = True; // booléen
$dm_only = True; // booléen
$no_dm = True; // booléen
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $after_id, $after_created_at, $unread_only, $dm_only, $no_dm, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---