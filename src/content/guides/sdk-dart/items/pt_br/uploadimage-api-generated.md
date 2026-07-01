Carregar e redimensionar uma imagem

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| sizePreset | string | query | Não | Tamanho predefinido: "Default" (1000x1000px) ou "CrossPlatform" (cria tamanhos para dispositivos populares) |
| urlId | string | query | Não | ID da página de onde o upload está sendo realizado, para configurar |

## Resposta

Retorna: `UploadImageResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Tamanho predefinido: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares)
final urlId = urlId_example; // String | ID da página de onde o upload está sendo realizado, para configurar

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]