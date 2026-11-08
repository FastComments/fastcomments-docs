SDK तीन API क्लाइंट क्लासेस उपलब्ध कराता है:

- **`DefaultApi`** - सर्वर-साइड उपयोग के लिए API-की-प्रमाणित मेथड्स। API की को [Getting Started](#getting-started-readme-generated) में दिखाए अनुसार कॉन्फ़िगर करें।
- **`PublicApi`** - सार्वजनिक मेथड्स जिन्हें API की की आवश्यकता नहीं है, जो ब्राउज़रों और मोबाइल ऐप्स से कॉल करने के लिए सुरक्षित हैं।
- **`ModerationApi`** - लाइव और तेज़ मॉडरेशन API का एक व्यापक सूट। प्रत्येक `ModerationApi` मेथड एक `$sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।

### PublicApi का उपयोग करना

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### ModerationApi का उपयोग करना

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - मॉडरेटर को प्रमाणित करने वाला SSO पेलोड

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```