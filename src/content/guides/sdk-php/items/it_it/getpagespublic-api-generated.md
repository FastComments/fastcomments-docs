Elenca le pagine di un tenant. Utilizzato dal client desktop FChat per popolare la lista delle sue stanze.
Richiede `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.
Le pagine che richiedono SSO vengono filtrate in base all'accesso di gruppo dell'utente richiedente.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursore opaco di paginazione restituito come `nextCursor` da una richiesta precedente. Vincolato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, predefinito 50 |
| q | string | query | No | Filtro opzionale per prefisso del titolo (insensibile alle maiuscole). |
| sortBy | string | query | No | Ordinamento. `updatedAt` (predefinito, prima i più recenti), `commentCount` (prima i più commentati), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Cursore opaco di paginazione restituito come `nextCursor` da una richiesta precedente. Vincolato allo stesso `sortBy`.
$limit = 56; // int | 1..200, predefinito 50
$q = 'q_example'; // string | Filtro opzionale per prefisso del titolo (insensibile alle maiuscole).
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Ordinamento. `updatedAt` (predefinito, prima i più recenti), `commentCount` (prima i più commentati), o `title` (alfabetico).
$has_comments = True; // bool | Se true, restituisce solo le pagine con almeno un commento.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]