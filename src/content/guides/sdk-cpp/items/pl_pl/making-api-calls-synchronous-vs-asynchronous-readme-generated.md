Wszystkie metody API w tym SDK zwracają `pplx::task<std::shared_ptr<ResponseType>>` z C++ REST SDK. Daje to elastyczność w obsłudze odpowiedzi API.

### Wywołania synchroniczne za pomocą `.get()`

Użyj `.get()`, aby zablokować wywołujący wątek aż do zakończenia żądania i pobrać wynik synchronicznie:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Wywołaj .get(), aby zablokować i pobrać wynik synchronicznie
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // strona
    boost::none,  // limit
    boost::none,  // pomiń
    boost::none,  // jako drzewo
    boost::none,  // pomiń potomków
    boost::none,  // limit potomków
    boost::none,  // maksymalna głębokość drzewa
    utility::conversions::to_string_t("your-url-id"),  // identyfikator URL
    boost::none,  // identyfikator użytkownika
    boost::none,  // identyfikator anonimowego użytkownika
    boost::none,  // identyfikator użytkownika kontekstu
    boost::none,  // hashtag
    boost::none,  // identyfikator rodzica
    boost::none   // kierunek
).get();  // Blokuje aż do zakończenia żądania HTTP

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Wywołania asynchroniczne za pomocą `.then()`

Użyj `.then()`, aby wykonać operacje asynchronicznie bez blokowania, korzystając z callbacków:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Użyj .then() do asynchronicznego wykonywania opartego na callbackach
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // To uruchamia się asynchronicznie po zakończeniu żądania
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Wykonanie kontynuuje natychmiast bez blokowania
std::cout << "Request sent, continuing..." << std::endl;
```

### Wybór między wywołaniem synchronicznym a asynchronicznym

Wybór zależy od środowiska uruchomieniowego i architektury aplikacji:

**`.get()` (blokujące, synchroniczne)**
- Blokuje wywołujący wątek aż do zakończenia żądania HTTP
- Prostszy przebieg kodu, łatwiej go zrozumieć
- Odpowiednie dla dedykowanych wątków roboczych, przetwarzania wsadowego lub narzędzi wiersza poleceń
- **Nieodpowiednie** dla pętli zdarzeń, wątków GUI lub serwerów jednowątkowych

**`.then()` (asynchroniczne, nieblokujące)**
- Zwraca natychmiast, callback wykonuje się po zakończeniu żądania
- Nie blokuje wywołującego wątku
- Wymagane w architekturach zdarzeniowych, aplikacjach GUI lub jednowątkowych pętlach zdarzeń
- Umożliwia łączenie wielu operacji
- Bardziej złożony przepływ sterowania

Zestaw testowy SDK używa wyłącznie `.get()`, co jest odpowiednie w środowisku testowym, gdzie blokowanie jest akceptowalne.