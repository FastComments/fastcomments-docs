## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | query | Tak |  |
| isLive | boolean | query | Nie |  |
| doSpamCheck | boolean | query | Nie |  |
| sendEmails | boolean | query | Nie |  |
| populateNotifications | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APISaveCommentResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład saveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz używać własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie użyty jako domyślny.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_comment_params = new \FastComments\Client\Model\CreateCommentParams(); // \FastComments\Client\Model\CreateCommentParams
$options = [
    'is_live' => True, // bool
    'do_spam_check' => True, // bool
    'send_emails' => True, // bool
    'populate_notifications' => True, // bool
];


try {
    $result = $apiInstance->saveComment($tenant_id, $create_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]