List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, default 50 |
| q | string | query | Não | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recente primeiro), `commentCount` (mais comentários primeiro) ou `title` (alfabético). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Response

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic Exemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recente primeiro), `commentCount` (mais comentários primeiro) ou `title` (alfabético).
    'has_comments' => True, // bool | Se true, retorna apenas páginas com pelo menos um comentário.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]