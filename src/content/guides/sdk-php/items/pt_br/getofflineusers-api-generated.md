Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Members".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|-------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no lado do servidor). |
| afterName | string | query | Não | Cursor: passe o nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate do cursor: passe o nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente http personalizado, passe o seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identificador da URL da página (limpo no lado do servidor).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: passe o nextAfterName da resposta anterior.
    'after_user_id' => 'after_user_id_example', // string | Desempate do cursor: passe o nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]