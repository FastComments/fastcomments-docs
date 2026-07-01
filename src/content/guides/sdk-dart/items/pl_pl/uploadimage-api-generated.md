Prześlij i zmień rozmiar obrazu

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| sizePreset | string | query | Nie | Rozmiar domyślny: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | Nie | Id strony, z której odbywa się przesyłanie, do konfiguracji |

## Odpowiedź

Zwraca: `UploadImageResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Size preset: \"Default\" (1000x1000px) or \"CrossPlatform\" (creates sizes for popular devices)
final urlId = urlId_example; // String | Page id that upload is happening from, to configure

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]