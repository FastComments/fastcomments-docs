---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| userId | string | Нет |  |
| direction | SortDirections | Нет |  |
| repliesToUserId | string | Нет |  |
| page | double | Нет |  |
| includei10n | bool | Нет |  |
| locale | string | Нет |  |
| isCrawler | bool | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsForUserResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример использования getCommentsForUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---