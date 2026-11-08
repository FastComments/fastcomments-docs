एक इमेज अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| file | HttpContent | हाँ |  |
| options | const UploadImageOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UploadImageResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'uploadImage उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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