## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odziv

Vrne: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesForUser200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer getVotesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // niz
$url_id = 'url_id_example'; // niz
$user_id = 'user_id_example'; // niz
$anon_user_id = 'anon_user_id_example'; // niz

try {
    $result = $apiInstance->getVotesForUser($tenant_id, $url_id, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]