Lista páginas para um tenant. Usada pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|-------|-------------|-----------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente http customizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`.
    'limit' => 56, // int | 1..200, padrão 50
    'q' => 'q_example', // string | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética).
    'has_comments' => True, // bool | Se true, retorna apenas páginas com pelo menos um comentário.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]