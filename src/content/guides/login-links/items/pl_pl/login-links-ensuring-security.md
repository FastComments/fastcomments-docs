Ponieważ linki logowania są w zasadzie hasłami, traktujemy bezpieczeństwo bardzo poważnie.

Wszystkie linki logowania w naszym systemie ustawione są tak, aby wygasły po określonym czasie, a także mamy mechanizmy wykrywające zgadywanie linku logowania. Niektóre linki logowania są podzielone na wiele haseł i jeśli jedno zostanie odgadnięte, pozostałe zostaną unieważnione.

### Bezpieczeństwo w porównaniu z hasłami

W większości systemów wymagających hasła można skorzystać z mechanizmu „Zapomniałeś hasła”, jeśli masz e-mail użytkownika. Oznacza to, że jeśli masz dostęp do konta e-mail użytkownika, nie ma znaczenia, czy atakowany system używa haseł, czy magicznych linków.

### Powiadomienia o logowaniu z nowego adresu IP

Gdy następuje logowanie z adresu IP, który wcześniej nie był używany dla danego konta, FastComments wysyła e-mail z alertem bezpieczeństwa zawierający przybliżoną lokalizację i adres IP. To pomaga użytkownikom wykryć nieautoryzowany dostęp. Należy pamiętać, że FastComments nie przechowuje surowych adresów IP — dla celów bezpieczeństwa przechowywana jest tylko zanonimizowana forma.

### E-mail zapasowy do odzyskiwania konta

Jeśli stracisz dostęp do swojego głównego e-maila, możesz użyć zweryfikowanego e-maila zapasowego do odzyskania konta. Twój e-mail zapasowy działa we wszystkich procesach logowania. Możesz go podać na stronie przypomnienia nazwy użytkownika, użyć go przy logowaniu za pomocą magicznego linku lub wpisać go w polu nazwy użytkownika/e-mail przy logowaniu hasłem.

Aby skonfigurować e-mail zapasowy, przejdź do [Account Details](https://fastcomments.com/auth/my-account/edit-details) i kliknij **Define a Backup Email**. Twój e-mail zapasowy jest używany wyłącznie do odzyskiwania konta i nie będzie otrzymywał powiadomień.

### Bezpieczeństwo w porównaniu z MFA

Linki logowania są mniej bezpieczne niż MFA. FastComments obsługuje teraz uwierzytelnianie dwuskładnikowe (2FA) dla kont administratorów, aby zapewnić zwiększone bezpieczeństwo. Gdy 2FA jest włączone, jest ono wymagane nawet przy użyciu linków logowania.