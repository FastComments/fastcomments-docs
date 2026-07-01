req  
tenantId  
afterId  

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Risposta

Restituisce: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getFeedPosts'; type = 'php'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
<?php  
require_once(__DIR__ . '/vendor/autoload.php');  

// Configura l'autorizzazione della chiave API: api_key  
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');  
// Decommenta qui sotto per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario  
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');  

$apiInstance = new FastComments\Client\Api\DefaultApi(  
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.  
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come predefinito.  
    new GuzzleHttp\Client(),  
    $config  
);  

$tenant_id = 'tenant_id_example'; // string  
$options = [  
    'after_id' => 'after_id_example', // string  
    'limit' => 56, // int  
    'tags' => array('tags_example'), // string[]  
];  

try {  
    $result = $apiInstance->getFeedPosts($tenant_id, $options);  
    print_r($result);  
} catch (Exception $e) {  
    echo 'Exception when calling DefaultApi->getFeedPosts: ', $e->getMessage(), PHP_EOL;  
}  
[inline-code-end]