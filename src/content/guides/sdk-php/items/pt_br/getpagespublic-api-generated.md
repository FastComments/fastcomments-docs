Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas. Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página. Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | No | 1..200, padrão 50 |
| q | string | query | No | Filtro opcional por prefixo de título, sem diferenciação entre maiúsculas e minúsculas. |
| sortBy | string | query | No | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). |
| hasComments | boolean | query | No | Se verdadeiro, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`.
$limit = 56; // int | 1..200, padrão 50
$q = 'q_example'; // string | Filtro opcional por prefixo de título, sem diferenciação entre maiúsculas e minúsculas.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética).
$has_comments = True; // bool | Se verdadeiro, retorna apenas páginas com pelo menos um comentário.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]