Currently‑online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room‑wide subscribers, including anon viewers we don't enumerate).

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não removam entradas. |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador da URL da página (limpo no servidor).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: passe nextAfterName da resposta anterior.
    'after_user_id' => 'after_user_id_example', // string | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não removam entradas.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]