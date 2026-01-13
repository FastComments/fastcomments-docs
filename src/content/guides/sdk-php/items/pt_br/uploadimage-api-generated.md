Enviar e redimensionar uma imagem

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-------------|
| tenantId | string | caminho | Sim |  |
| sizePreset | string | consulta | Não | Predefinição de tamanho: "Default" (1000x1000px) ou "CrossPlatform" (cria tamanhos para dispositivos populares) |
| urlId | string | consulta | Não | ID da página de onde o upload está sendo realizado, para configuração |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional; `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares)
$url_id = 'url_id_example'; // string | ID da página de onde o upload está sendo realizado, para configuração

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]