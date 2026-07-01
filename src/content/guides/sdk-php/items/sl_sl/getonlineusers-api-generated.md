Trenutno‑online gledalci strani: ljudje, katerih websocket seja je v tem trenutku naročena na stran.
Vrne anonCount + totalCount (naročniki po celotni sobi, vključno z anonimnimi gledalci, ki jih ne izpisujemo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (čiščen na strežniku). |
| afterName | string | query | No | Kazalec: pošljite nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Prekinitveni kazalec: pošljite nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti svoj HTTP odjemalec, podajte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL strani (čiščen na strežniku).
$options = [
    'after_name' => 'after_name_example', // string | Kazalec: pošljite nextAfterName iz prejšnjega odgovora.
    'after_user_id' => 'after_user_id_example', // string | Prekinitveni kazalec: pošljite nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]