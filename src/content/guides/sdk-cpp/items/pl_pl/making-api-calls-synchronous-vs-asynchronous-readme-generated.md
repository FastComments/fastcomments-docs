Wszystkie metody API w tym SDK zwracają `pplx::task<std::shared_ptr<ResponseType>>` z C++ REST SDK. Daje to elastyczność w tym, jak obsługujesz odpowiedzi API.

### Wywołania synchroniczne przy użyciu `.get()`

Użyj `.get()`, aby zablokować wywołujący wątek aż żądanie zostanie zakończone i pobrać wynik synchronicznie:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokuje do momentu zakończenia żądania HTTP

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Wywołania asynchroniczne przy użyciu `.then()`

Użyj `.then()`, aby wykonać nieblokujące asynchroniczne wywołanie z funkcjami zwrotnymi:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // To zostaje wykonane asynchronicznie po zakończeniu żądania
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;  // Wykonanie kontynuuje się natychmiast bez blokowania
```

### Wybór pomiędzy synchronicznym a asynchronicznym

Wybór zależy od środowiska uruchomieniowego i architektury aplikacji:

**`.get()` (Synchronous blocking)**
- Blokuje wywołujący wątek aż żądanie HTTP zostanie zakończone
- Prostszy przepływ kodu, łatwiejszy do zrozumienia
- Odpowiedni dla dedykowanych wątków roboczych, przetwarzania wsadowego lub narzędzi wiersza poleceń
- **Nieodpowiedni** dla pętli zdarzeń, wątków GUI lub serwerów jednowątkowych

**`.then()` (Asynchronous non-blocking)**
- Zwraca natychmiast, funkcja zwrotna jest wywoływana po zakończeniu żądania
- Nie blokuje wywołującego wątku
- Wymagany w architekturach zdarzeniowych, aplikacjach GUI lub jednowątkowych pętlach zdarzeń
- Umożliwia łączenie wielu operacji
- Bardziej złożony przepływ sterowania

Zestaw testów SDK używa wyłącznie `.get()`, ale jest to odpowiednie dla środowiska testowego, w którym blokowanie jest dopuszczalne.