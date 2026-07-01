## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Odgovor

Vrne: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APISaveCommentResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer saveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lasten HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen privzeto.
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