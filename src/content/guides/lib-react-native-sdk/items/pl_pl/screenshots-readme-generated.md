#### Skórka: Erebus
![Skórka: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skórka: Default
![Skórka: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Natywny edytor WYSIWYG z obsługą obrazów!
![Natywny edytor WYSIWYG z obsługą obrazów](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Edytor bogatego tekstu

Ta biblioteka używa edytora 10tap do funkcjonalności edycji bogatego tekstu, który zapewnia potężne możliwości edycji WYSIWYG.

### Opcje konfiguracji

Ta biblioteka ma na celu wspieranie wszystkich opcji konfiguracji zdefiniowanych w [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tak jak implementacja webowa.

### Koncepcje FastComments

Główne pojęcia, które należy znać, aby zacząć to `tenantId` i `urlId`. `tenantId` to identyfikator Twojego konta FastComments.com. `urlId` to miejsce, do którego powiązane będą wątki komentarzy. Może to być adres strony, identyfikator produktu, identyfikator artykułu itp.

### Powiadomienia użytkownika

FastComments obsługuje powiadomienia dla [wielu scenariuszy](https://docs.fastcomments.com/guide-notifications.html). Powiadomienia są konfigurowalne, można z nich zrezygnować globalnie lub na poziomie konkretnego powiadomienia/komentarza oraz obsługują subskrypcje na poziomie strony, dzięki czemu użytkownicy mogą subskrybować wątki konkretnej strony lub artykułu.

Na przykład można użyć Secure SSO do uwierzytelnienia użytkownika, a następnie okresowo sprawdzać nieprzeczytane powiadomienia i wysyłać je do użytkownika.

Zobacz [przykład AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), aby dowiedzieć się, jak pobierać i przetwarzać nieprzeczytane powiadomienia użytkownika.

### Przeglądarka GIF-ów

Domyślnie nie jest włączony żaden wybór obrazów ani GIF-ów. Zobacz [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), aby dowiedzieć się, jak obsługiwać przesyłanie obrazów i GIF-ów. W tej bibliotece dostępna jest Przeglądarka GIF-ów, która anonimizuje wyszukiwania i obrazy udostępniane w tej bibliotece — wystarczy z niej skorzystać.

### Wydajność

Prosimy o otwarcie zgłoszenia z przykładem reprodukcji, w tym używanym urządzeniem, jeśli zauważysz jakiekolwiek problemy z wydajnością. Wydajność jest priorytetem we wszystkich bibliotekach FastComments.