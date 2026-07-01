## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | consulta | Sim |  |
| id | string | caminho | Sim |  |

## Resposta

Retorna: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getQuestionResult'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura a autorização da chave API: api_key
// Descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desejar usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getQuestionResult($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]