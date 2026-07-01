Trenutno online gledatelji stranice: ljudi čija je WebSocket sesija pretplaćena na stranicu u ovom trenutku.  
Vraća anonCount + totalCount (pretplatnici na cijelu sobu, uključujući anonimne gledatelje koje ne nabrajamo).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrješivač nerazlučivosti: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljeno afterName kako bi se izbjeglo izostavljanje zapisa uslijed istih imena. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadani.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor razrješivač nerazlučivosti: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljeno afterName kako bi se izbjeglo izostavljanje zapisa uslijed istih imena.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---