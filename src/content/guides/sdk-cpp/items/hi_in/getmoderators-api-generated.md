## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetModeratorsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getModerators उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getModerators(tenantId, skip).then([](pplx::task<std::shared_ptr<GetModeratorsResponse>> t){
    auto response = t.get();
});
[inline-code-end]