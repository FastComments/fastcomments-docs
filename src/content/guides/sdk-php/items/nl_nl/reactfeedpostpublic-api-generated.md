## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| isUndo | boolean | query | Nee |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'reactFeedPostPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als je een aangepaste HTTP-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` zal als standaard worden gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$options = [
    'is_undo' => True, // bool
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]