Ponieważ linki logowania są zasadniczo hasłami, traktujemy bezpieczeństwo bardzo poważnie.

Wszystkie linki logowania w naszym systemie są ustawione tak, aby wygasały po określonym czasie, a także mamy mechanizmy wykrywające
odgadywanie linku logowania. Niektóre linki logowania są podzielone na wiele haseł, i jeśli jedno zostanie odgadnięte,
drugie zostanie unieważnione.

### Bezpieczeństwo w porównaniu do haseł

W większości systemów wymagających hasła można skorzystać z mechanizmu zapomnianego hasła
jeśli masz adres e‑mail użytkownika. Oznacza to, że jeśli masz dostęp do konta e‑mail użytkownika,
nie ma znaczenia, czy system będący celem ataku używa haseł czy magicznych linków.

### Powiadomienia o logowaniu z nowego adresu IP

Gdy logowanie nastąpi z adresu IP, który wcześniej nie był widziany dla danego konta, FastComments wysyła e‑mail z alertem bezpieczeństwa
z przybliżoną lokalizacją i adresem IP. Pomaga to użytkownikom wykryć nieautoryzowany dostęp. Należy pamiętać, że FastComments nie przechowuje
surowych adresów IP — przechowywana jest tylko zanonimizowana postać ze względów bezpieczeństwa.

### Bezpieczeństwo w porównaniu do MFA

Linki logowania są mniej bezpieczne niż MFA. FastComments obsługuje teraz uwierzytelnianie dwuskładnikowe (2FA)
dla kont administratorów, aby zapewnić zwiększone bezpieczeństwo. Gdy 2FA jest włączone, jest ono wymagane nawet podczas korzystania z linków logowania.