Currently‑online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room‑wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|------------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore per risolvere il pareggio: passa nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato così i pareggi di nome non eliminano voci. |

## Response

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // stringa
$url_id = 'url_id_example'; // stringa | Identificatore URL della pagina (pulito lato server).
$options = [
    'after_name' => 'after_name_example', // stringa | Cursore: passa nextAfterName dalla risposta precedente.
    'after_user_id' => 'after_user_id_example', // stringa | Cursore per risolvere il pareggio: passa nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato così i pareggi di nome non eliminano voci.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]