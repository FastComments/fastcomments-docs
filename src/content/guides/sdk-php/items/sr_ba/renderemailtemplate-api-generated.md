## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vraća: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RenderEmailTemplateResponse.php)

## Primjer

[inline-code-attrs-start title = 'renderEmailTemplate Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfigurišite autorizaciju API ključa: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ako želite koristiti prilagođeni http klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ovo je opcionalno, `GuzzleHttp\Client` će biti korišten kao podrazumevani.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$render_email_template_body = new \FastComments\Client\Model\RenderEmailTemplateBody(); // \FastComments\Client\Model\RenderEmailTemplateBody
$locale = 'locale_example'; // string


try {
    $result = $apiInstance->renderEmailTemplate($tenant_id, $render_email_template_body, $locale);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->renderEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]