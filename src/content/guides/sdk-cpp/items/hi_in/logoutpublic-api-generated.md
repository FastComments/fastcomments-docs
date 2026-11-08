## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'logoutPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->logoutPublic()
    .then([](std::shared_ptr<APIEmptyResponse> /*resp*/) {
        std::cout << "Logout successful\n";
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception& e) {
            std::cerr << "Logout failed: " << e.what() << std::endl;
        }
    });
[inline-code-end]