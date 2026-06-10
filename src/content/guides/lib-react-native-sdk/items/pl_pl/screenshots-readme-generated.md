#### Skórka: Erebus
![Skórka: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skórka: Domyślna
![Skórka: Domyślna](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Natywny edytor WYSIWYG z obsługą obrazów!
![Natywny edytor WYSIWYG z obsługą obrazów](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Edytor bogatego tekstu

Ta biblioteka używa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) do edycji bogatego tekstu, co zapewnia potężne doświadczenie edytora WYSIWYG. Ten sam edytor zasila iOS, Androida oraz web (poprzez `react-native-web`), więc komponowanie zachowuje się spójnie na wszystkich platformach dzięki jednej implementacji.

`react-native-enriched` wymaga React Native New Architecture (Fabric) na warstwie natywnej oraz bundlera, który rozwiązuje warunki eksportów pakietu (Metro z package exports / RN 0.72+). Wsparcie dla weba jest obecnie eksperymentalne.

### Opcje konfiguracji

Ta biblioteka ma na celu wspieranie wszystkich opcji konfiguracji zdefiniowanych w [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), podobnie jak implementacja webowa.

### Podstawowe pojęcia FastComments

Główne pojęcia, które warto znać na początek to `tenantId` i `urlId`. `tenantId` to identyfikator Twojego konta FastComments.com. `urlId` to miejsce, do którego będą przypisane wątki komentarzy. Może to być adres URL strony, identyfikator produktu, identyfikator artykułu itp.

### Powiadomienia użytkownika

FastComments obsługuje powiadomienia dla [wielu scenariuszy](https://docs.fastcomments.com/guide-notifications.html). Powiadomienia są konfigurowalne, można z nich zrezygnować globalnie lub na poziomie powiadomienia/komentarza, oraz obsługują subskrypcje na poziomie strony, dzięki czemu użytkownicy mogą subskrybować wątki konkretnej strony lub artykułu.

Na przykład można użyć Secure SSO do uwierzytelnienia użytkownika, a następnie okresowo sprawdzać nieprzeczytane powiadomienia i wysyłać je do użytkownika.

Zobacz [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), aby dowiedzieć się, jak pobierać i tłumaczyć nieprzeczytane powiadomienia użytkownika.

### Przeglądarka GIF-ów

Domyślnie nie jest włączony wybór obrazów ani gifów. Zobacz [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), aby dowiedzieć się, jak obsługiwać przesyłanie obrazów i gifów. W bibliotece dostępna jest przeglądarka GIF-ów, która anonimizuje wyszukiwania i udostępniane obrazy — wystarczy z niej skorzystać.

### Wydajność

Prosimy o zgłoszenie problemu (ticket) z przykładem umożliwiającym reprodukcję, wraz z informacją o używanym urządzeniu, jeśli zauważysz problemy z wydajnością. Wydajność jest priorytetem we wszystkich bibliotekach FastComments.