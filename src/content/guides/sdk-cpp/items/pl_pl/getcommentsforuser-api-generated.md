## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| userId | string | Nie |  |
| direction | SortDirections | Nie |  |
| repliesToUserId | string | Nie |  |
| page | double | Nie |  |
| includei10n | bool | Nie |  |
| locale | string | Nie |  |
| isCrawler | bool | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsForUserResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsForUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<SortDirections> direction = SortDirections::DESC;
boost::optional<utility::string_t> repliesToUserId = utility::string_t(U("author-456"));
boost::optional<double> page = 1.0;
boost::optional<bool> includei10n = true;
boost::optional<utility::string_t> locale = utility::string_t(U("en-US"));
boost::optional<bool> isCrawler = false;
api->getCommentsForUser(userId, direction, repliesToUserId, page, includei10n, locale, isCrawler)
.then([](pplx::task<std::shared_ptr<GetCommentsForUserResponse>> task){
  try {
    auto resp = task.get();
    auto marker = std::make_shared<std::string>("comments-retrieved");
    if (resp) std::cout << "Comments fetched for user\n";
  } catch (const std::exception &e) {
    std::cerr << "Error fetching comments: " << e.what() << "\n";
  }
});
[inline-code-end]