## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-------------|
| tenantId | string | query | Sim |  |
| commentIds | string | query | Sim | Uma lista separada por vírgulas de IDs de comentário. |
| sso | string | query | Não |  |

## Resposta

Retorna: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Uma lista separada por vírgulas de IDs de comentário.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]