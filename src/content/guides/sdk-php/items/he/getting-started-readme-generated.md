אנא פעל לפי [הליך ההתקנה](#installation-usage-readme-generated) ואז הרץ את הקוד הבא:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// הגדר הרשאת מפתח ה-API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה בשורה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}

```