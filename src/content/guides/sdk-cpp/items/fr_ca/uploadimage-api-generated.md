Téléverser et redimensionner une image

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| file | HttpContent | Oui |  |
| sizePreset | SizePreset | Non |  |
| urlId | string | Non |  |

## Réponse

Renvoie : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de uploadImage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto fileBytes = std::vector<unsigned char>{0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A};
auto file = std::make_shared<HttpContent>(fileBytes, U("image/png"), U("avatar.png"));
boost::optional<SizePreset> sizePreset = boost::optional<SizePreset>(SizePreset::MEDIUM);
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("user-avatar-987"));
api->uploadImage(tenantId, file, sizePreset, urlId)
    .then([](pplx::task<std::shared_ptr<UploadImageResponse>> task) {
        try {
            return task.get();
        } catch (...) {
            return std::shared_ptr<UploadImageResponse>();
        }
    });
[inline-code-end]