## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotificationCount200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---