Informações em massa de usuários para um tenant. Dados os userIds, retorna informações de exibição de User / SSOUser.
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.
Sem contexto de página: a privacidade é aplicada de forma uniforme (perfis privados são mascarados).

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|-------|------------|-----------|
| tenantId | string | path | Sim |  |
| ids | string | query | Sim | userIds separados por vírgula. |

## Resposta

Retorna: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds separados por vírgula.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]