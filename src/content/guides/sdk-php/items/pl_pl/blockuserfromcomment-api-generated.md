## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |
| id | string | ścieżka | Tak |  |
| userId | string | zapytanie | Nie |  |
| anonUserId | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`BlockSuccess`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BlockSuccess.php)

## Przykład

[inline-code-attrs-start title = 'Przykład blockUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta http, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie używany będzie `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$block_from_comment_params = new \FastComments\Client\Model\BlockFromCommentParams(); // \FastComments\Client\Model\BlockFromCommentParams
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string

try {
    $result = $apiInstance->blockUserFromComment($tenant_id, $id, $block_from_comment_params, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->blockUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]