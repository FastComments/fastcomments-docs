## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeTicketState200Response.php)

## Przykład

[inline-code-attrs-start title = 'changeTicketState Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację kluczem API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniższą linię, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie będzie używany `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$id = 'id_example'; // string
$change_ticket_state_body = new \FastComments\Client\Model\ChangeTicketStateBody(); // \FastComments\Client\Model\ChangeTicketStateBody

try {
    $result = $apiInstance->changeTicketState($tenant_id, $user_id, $id, $change_ticket_state_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->changeTicketState: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]