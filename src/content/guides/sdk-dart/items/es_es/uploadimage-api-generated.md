Cargar y redimensionar una imagen

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| sizePreset | string | query | No | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea tamaños para dispositivos populares) |
| urlId | string | query | No | Id de página desde la que se está cargando, para configurar |

## Respuesta

Devuelve: `UploadImageResponse`

## Ejemplo

[inline-code-attrs-start title = 'uploadImage Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea tamaños para dispositivos populares)
final urlId = urlId_example; // String | Id de página desde la que se está cargando, para configurar

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]