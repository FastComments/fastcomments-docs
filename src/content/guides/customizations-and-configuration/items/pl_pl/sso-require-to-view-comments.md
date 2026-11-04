FastComments SSO (<a href="#sso">szczegóły tutaj</a>) umożliwia Twoim użytkownikom komentowanie bez konieczności logowania się na innej platformie.

Jednak samo to nie zabezpiecza Twoich wątków komentarzy, ponieważ domyślnie dane komentarzy są publicznie dostępnymi informacjami - każdy, kto może wyświetlić stronę, może zobaczyć komentarze.

Zmieniając jedno ustawienie, możemy ograniczyć pobieranie komentarzy tak, aby było to możliwe tylko dla administratora lub ważnego użytkownika SSO.

#### Konfiguracja bez kodu

Możemy uniemożliwić przeglądanie i interakcję z naszymi wątkami komentarzy, gdy SSO jest skonfigurowane, tworząc <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regułę dostosowania</a>.

Podczas tego procesu wyszukaj SSO, a znajdziesz tę opcję:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opcja Wymagaj SSO do wyświetlania komentarzy włączona w regule dostosowania, z wyborem poziomu zabezpieczeń'; title='Wymagaj SSO do wyświetlania komentarzy' app-screenshot-end]

Włącz ją i zapisz regułę dostosowania.

#### Ochrona tylko określonej domeny lub strony

Aby chronić tylko określoną domenę lub stronę, po prostu skonfigurujemy w tym celu regułę dostosowania.

Na górze interfejsu dostosowania znajdziemy dwa pola: Domain i URL ID.

Aby chronić konkretną domenę, wpisz daną domenę w polu "domain".

Aby chronić konkretną stronę, wpisz adres URL strony w polu "URL ID". Jeśli masz niestandardową integrację z FastComments, możesz zamiast adresu URL wpisać tutaj rodzaj identyfikatora.

#### Poziomy zabezpieczeń

Przy wymaganiu SSO musisz zdecydować, czy wymagasz Simple SSO czy Secure SSO. Jeśli wymagasz Simple SSO, to oba są dozwolone, ale jeśli wymagasz Secure SSO, to
treść musi zostać pobrana z ładunkiem Secure SSO zaszyfrowanym Twoim kluczem API, aby mogła zostać wyświetlona.

Opcja poziomu zabezpieczeń pojawi się po wybraniu "Wymagaj SSO do wyświetlania komentarzy" i działa tylko wtedy, gdy ta opcja jest włączona.

Ustawienia <a href="#sso-simple">Automatyczna weryfikacja komentarzy przez Simple SSO i Zezwalaj administratorom Simple SSO</a> są osobne i obowiązują niezależnie od tego, czy ta opcja jest włączona.

#### Ochrona wykraczająca poza odczyt

Włączenie tej opcji chroni stronę lub domenę przed komentowaniem, chyba że użytkownik jest zalogowany przez SSO.

#### Pułapki

Użytkownicy, którzy utworzyli komentarze przed integracją SSO, nie będą mogli ich zobaczyć, chyba że zalogują się przez Twoją integrację SSO.