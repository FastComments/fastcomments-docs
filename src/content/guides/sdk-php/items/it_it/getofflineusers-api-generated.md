Commentatori passati sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usa questo dopo aver esaurito /users/online per visualizzare una sezione "Membri".
Paginazione cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza il costo di $skip.

## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (sanificato lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Risolutore di parità del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere le voci. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificatore dell'URL della pagina (sanificato lato server).
$after_name = 'after_name_example'; // string | Cursore: passare nextAfterName dalla risposta precedente.
$after_user_id = 'after_user_id_example'; // string | Risolutore di parità del cursore: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere le voci.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]