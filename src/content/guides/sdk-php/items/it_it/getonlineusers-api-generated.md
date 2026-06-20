Attualmente spettatori online di una pagina: persone la cui sessione websocket è attualmente iscritta alla pagina.
Restituisce anonCount + totalCount (iscritti a livello di stanza, inclusi gli spettatori anonimi che non elenchiamo).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore tie-breaker: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che le occorrenze con lo stesso nome non vengano scartate. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è facoltativo, `GuzzleHttp\Client` verrà usato di default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificatore dell'URL della pagina (ripulito lato server).
$after_name = 'after_name_example'; // string | Cursore: passa nextAfterName dalla risposta precedente.
$after_user_id = 'after_user_id_example'; // string | Cursore tie-breaker: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che le occorrenze con lo stesso nome non vengano scartate.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]