Atualmente-online visualizadores de uma página: pessoas cuja sessão websocket está inscrita na página neste momento.
Retorna anonCount + totalCount (assinantes em toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não eliminem entradas. |

## Response

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional; `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador da URL da página (limpo no servidor).
$after_name = 'after_name_example'; // string | Cursor: passe nextAfterName da resposta anterior.
$after_user_id = 'after_user_id_example'; // string | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não eliminem entradas.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]