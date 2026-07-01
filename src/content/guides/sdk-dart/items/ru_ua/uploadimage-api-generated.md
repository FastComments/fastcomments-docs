Завантажити та змінити розмір зображення

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Попереднє розмір: "Default" (1000x1000px) або "CrossPlatform" (створює розміри для популярних пристроїв) |
| urlId | string | query | No | ID сторінки, з якої здійснюється завантаження, для налаштування |

## Відповідь

Повертає: `UploadImageResponse`

## Приклад

[inline-code-attrs-start title = 'uploadImage Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Попереднє розмір: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв)
final urlId = urlId_example; // String | ID сторінки, з якої здійснюється завантаження, для налаштування

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]