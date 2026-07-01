## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|------------|--------|-------------|
| tenantId | string | requête | Oui |  |
| sso | string | requête | Non |  |

## Réponse

Renvoie: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIModerateGetUserBanPreferencesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserBanPreference'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // chaîne
$sso = 'sso_example'; // chaîne


try {
    $result = $apiInstance->getUserBanPreference($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserBanPreference: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---