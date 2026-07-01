## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Vrne: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCachedNotificationCountResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getCachedNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti uporabniški http klient, podajte svoj klient, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getCachedNotificationCount($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getCachedNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]