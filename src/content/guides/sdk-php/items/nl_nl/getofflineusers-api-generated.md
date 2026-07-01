Past commenters op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.  
Gebruik dit nadat /users/online is uitgeput om een “Members” sectie weer te geven.  
Cursor‑paginering op commenterName: server doorloopt de gedeeltelijke {tenantId, urlId, commenterName} index vanaf afterName vooruit via $gt, zonder $skip‑kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina‑URL‑identifier (aan de serverzijde opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door uit de vorige respons. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑gelijkstanden geen items laten vallen. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als je een aangepaste http‑client wilt gebruiken, geef je client mee die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Pagina‑URL‑identifier (aan de serverzijde opgeschoond).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: geef nextAfterName door uit de vorige respons.
    'after_user_id' => 'after_user_id_example', // string | Cursor tiebreaker: geef nextAfterUserId door uit de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑gelijkstanden geen items laten vallen.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]