## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| deleteComments | boolean | query | Nein |  |
| commentDeleteMode | string | query | Nein |  |

## Response

Returns: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Example

[inline-code-attrs-start title = 'deleteSSOUser Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API-Schlüssel‑Autorisierung konfigurieren: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API‑Schlüssel einzurichten, falls nötig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Wenn Sie einen benutzerdefinierten HTTP‑Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$tenant_id = 'tenant_id_example'; // Zeichenkette
$id = 'id_example'; // string
$id = 'id_example'; // Zeichenkette
$options = [
    'delete_comments' => True, // bool
    'delete_comments' => True, // bool
    'comment_delete_mode' => 'comment_delete_mode_example', // string
    'comment_delete_mode' => 'comment_delete_mode_example', // Zeichenkette
];


try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]