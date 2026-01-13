---
#### Skórka: Erebus
![Skórka: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Skórka: Domyślna
![Skórka: Domyślna](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Natywny edytor WYSIWYG z obsługą obrazów!
![Natywny edytor WYSIWYG z obsługą obrazów](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Edytor tekstu sformatowanego

Ta biblioteka wykorzystuje edytor 10tap do funkcji edycji tekstu sformatowanego, który zapewnia potężne doświadczenie edycji WYSIWYG.

### Opcje konfiguracji

Ta biblioteka ma na celu wspieranie wszystkich opcji konfiguracji zdefiniowanych w [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), podobnie jak implementacja webowa.

### Koncepcje FastComments

Główne pojęcia, które warto znać, aby zacząć, to `tenantId` i `urlId`. `tenantId` to identyfikator Twojego konta FastComments.com. `urlId` to miejsce, do którego będą powiązane wątki komentarzy. Może to być adres URL strony, identyfikator produktu, identyfikator artykułu itp.

### Powiadomienia użytkowników

FastComments obsługuje powiadomienia dla [wielu scenariuszy](https://docs.fastcomments.com/guide-notifications.html). Powiadomienia są konfigurowalne, można z nich zrezygnować globalnie lub na poziomie pojedynczego powiadomienia/komentarza, oraz obsługują subskrypcje na poziomie strony, dzięki czemu użytkownicy mogą subskrybować wątki konkretnej strony lub artykułu.

Na przykład możliwe jest użycie Secure SSO do uwierzytelnienia użytkownika, a następnie okresowe sprawdzanie nieprzeczytanych powiadomień i wysyłanie ich do użytkownika.

Zobacz [przykład AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), aby dowiedzieć się, jak pobrać i przetworzyć nieprzeczytane powiadomienia użytkownika.

### Przeglądarka GIF-ów

Domyślnie nie jest włączony wybór obrazów ani gifów. Zobacz [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), aby dowiedzieć się, jak obsługiwać przesyłanie obrazów i gifów. W bibliotece znajduje się Przeglądarka GIF-ów, która anonimizuje wyszukiwania i obrazy udostępnione w tej bibliotece — wystarczy jej użyć.

### Wydajność

Prosimy o otwarcie zgłoszenia z przykładem umożliwiającym odtworzenie problemu, w tym z informacją o używanym urządzeniu, jeśli zauważysz jakiekolwiek problemy z wydajnością. Wydajność jest priorytetem we wszystkich bibliotekach FastComments.
---