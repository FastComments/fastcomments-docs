The **FastComments** block obsługuje logowanie jednokrotne (SSO), dzięki czemu Twoi klienci Shopify mogą komentować jako oni sami bez tworzenia oddzielnego konta FastComments.

### Jak to działa

Gdy odwiedzający, który jest zalogowany w Twoim sklepie, otworzy stronę z blokiem **FastComments**:

1. Blok wykrywa obiekt Shopify `customer`.
2. Wysyła imię i adres e-mail klienta do FastComments przez podpisane żądanie proxy aplikacji.
3. FastComments tworzy lub dopasowuje użytkownika o kluczu `shopify-{customerId}`, dzięki czemu ten sam klient zawsze odwzorowuje się na tego samego użytkownika FastComments pomiędzy sesjami i reinstalacjami.
4. Imię odwiedzającego pojawia się przy jego komentarzach. Nie jest proszony o ponowne zalogowanie.

Jeśli odwiedzający nie jest zalogowany w sklepie, blok przechodzi do anonimowego komentowania (lub do procesu logowania FastComments, w zależności od konfiguracji widżetu).

### Wyłączanie SSO

SSO jest domyślnie włączone dla każdego bloku **FastComments**. Aby wyłączyć je dla konkretnego bloku:

1. Otwórz edytor motywu Shopify.
2. Otwórz szablon zawierający blok i kliknij blok, aby go wybrać.
3. Odznacz **SSO**.
4. Kliknij **Zapisz**.

Wyłącz SSO, jeśli chcesz, aby komentujący wybierali oddzielną tożsamość w obrębie dyskusji. Na przykład na wewnętrznej stronie społeczności, gdzie pracownicy komentują pod inną nazwą wyświetlaną.

### Co otrzymuje FastComments

Ładunek SSO wysyłany dla każdego klienta zawiera:

- ID użytkownika pochodzące z ID klienta Shopify (`shopify-{customerId}`).
- Adres e-mail klienta (używany do identyfikacji użytkownika; nie jest wyświetlany publicznie).
- Nazwa wyświetlana klienta (używana jako nazwa autora komentarza).

Nie są wysyłane dane historii zamówień, płatności ani adresu. Ładunek jest podpisywany po stronie serwera; przeglądarka klienta nigdy nie widzi poświadczenia.

### Linki logowania i wylogowania

Po włączeniu SSO linki do logowania i wylogowania widżetu komentarzy kierują na `/account/login` i `/account/logout`, standardowe ścieżki kont klientów Shopify. Nie ma nic do skonfigurowania. Linki działają w każdym sklepie z włączonymi kontami klientów.