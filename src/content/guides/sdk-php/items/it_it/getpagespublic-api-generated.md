List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.  
Le pagine che richiedono SSO sono filtrate in base all'accesso del gruppo dell'utente richiedente.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Filtro opzionale del prefisso del titolo, non sensibile al maiuscolo/minuscolo. |
| sortBy | string | query | No | Ordine di ordinamento. `updatedAt` (predefinito, più recenti per primi), `commentCount` (più commenti per primi) o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se desideri utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Cursor di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Filtro opzionale del prefisso del titolo, non sensibile al maiuscolo/minuscolo.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ordine di ordinamento. `updatedAt` (predefinito, più recenti per primi), `commentCount` (più commenti per primi) o `title` (alfabetico).
    'has_comments' => True, // bool | Se true, restituisce solo le pagine con almeno un commento.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]