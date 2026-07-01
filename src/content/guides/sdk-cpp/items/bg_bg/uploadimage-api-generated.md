Upload and resize an image
# Качване и преоразмеряване на изображение

## Parameters
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | HttpContent | Yes |  |
| options | const UploadImageOptions& | Yes |  |

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| file | HttpContent | Да |  |
| options | const UploadImageOptions& | Да |  |

## Response
## Отговор

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)
Връща: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)

## Example
## Пример

[inline-code-attrs-start title = 'Пример за uploadImage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto fileStream = concurrency::streams::fstream::open_istream(U("avatar.png"), std::ios::in).get();
HttpContent file(fileStream, U("image/png"));
UploadImageOptions options;
options.description = boost::optional<utility::string_t>(U("Profile picture"));
options.width = boost::optional<int>(256);
options.height = boost::optional<int>(256);
api->uploadImage(U("my-tenant-123"), file, options).then([](pplx::task<std::shared_ptr<UploadImageResponse>> t){
    auto resp = t.get();
});
[inline-code-end]