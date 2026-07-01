Загрузка и изменение размера изображения

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | HttpContent | Yes |  |
| options | const UploadImageOptions& | Yes |  |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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