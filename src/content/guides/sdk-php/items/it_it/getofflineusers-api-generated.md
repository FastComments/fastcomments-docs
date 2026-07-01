Past commentatori sulla pagina che NON sono attualmente online. Ordinati per **displayName**.  
Usa questo dopo aver esaurito `/users/online` per rendere una sezione “Members”.  
Paginazione con cursore su **commenterName**: il server scorre il sotto‑insieme `{tenantId, urlId, commenterName}` a partire da **afterName** in avanti tramite `$gt`, senza alcun costo `$skip`.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore della URL della pagina (ripulito sul server). |
| afterName | string | query | No | Cursore: passa **nextAfterName** dalla risposta precedente. |
| afterUserId | string | query | No | Cursore per risolvere i pareggi: passa **nextAfterUserId** dalla risposta precedente. Necessario quando **afterName** è impostato affinché i pareggi di nome non omettano voci. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificatore della URL della pagina (ripulito sul server).
$options = [
    'after_name' => 'after_name_example', // string | Cursore: passa **nextAfterName** dalla risposta precedente.
    'after_user_id' => 'after_user_id_example', // string | Cursore per risolvere i pareggi: passa **nextAfterUserId** dalla risposta precedente. Necessario quando **afterName** è impostato affinché i pareggi di nome non omettano voci.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Eccezione durante la chiamata a PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]