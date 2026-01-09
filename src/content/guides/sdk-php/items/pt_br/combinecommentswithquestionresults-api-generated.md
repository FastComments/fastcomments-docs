## Parâmetros

| Nome | Type | Location | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| questionId | string | query | Não |  |
| questionIds | array | query | Não |  |
| urlId | string | query | Não |  |
| startDate | string | query | Não |  |
| forceRecalculate | boolean | query | Não |  |
| minValue | number | query | Não |  |
| maxValue | number | query | Não |  |
| limit | number | query | Não |  |

## Resposta

Retorna: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineCommentsWithQuestionResults200Response.php)

## Exemplo

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Exemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização da chave da API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abaixo para configurar o prefixo (por exemplo, Bearer) para a chave da API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = array('question_ids_example'); // string[]
$url_id = 'url_id_example'; // string
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // bool
$min_value = 3.4; // float
$max_value = 3.4; // float
$limit = 3.4; // float

try {
    $result = $apiInstance->combineCommentsWithQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $start_date, $force_recalculate, $min_value, $max_value, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->combineCommentsWithQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]