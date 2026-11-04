FastComments SSO (<a href="#sso">szczegóły tutaj</a>) zapewnia użytkownikom możliwość komentowania bez konieczności logowania się na inną platformę.

Jednak samo to nie zabezpiecza wątków komentarzy, ponieważ domyślnie dane komentarzy są publicznie dostępne – każdy, kto może zobaczyć stronę, może zobaczyć komentarze.

Zmieniąc ustawienie, możemy ograniczyć pobieranie komentarzy wyłącznie do administratora lub ważnego użytkownika SSO.

#### Konfiguracja bez kodu

Możemy zapobiec przeglądaniu i interakcjom z wątkami komentarzy, gdy SSO jest skonfigurowane, tworząc <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regułę dostosowywania</a>.

W tym celu wyszukaj SSO, a znajdziesz tę opcję:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Włączona opcja wymogu SSO do wyświetlania komentarzy w regule dostosowywania, z wyborem poziomu bezpieczeństwa'; title='Wymagaj SSO, aby wyświetlać komentarze' app-screenshot-end]

Włącz ją i zapisz regułę dostosowywania.

#### Ochrona tylko określonej domeny lub strony

Aby chronić tylko określoną domenę lub stronę, po prostu skonfigurujemy regułę dostosowywania.

Na górze interfejsu dostosowywania znajdziemy dwa pola wejściowe: Domena i ID URL.

Aby chronić konkretną domenę, wprowadź ją w polu „domena”.

Aby chronić konkretną stronę, wprowadź adres URL strony w polu „ID URL”. Jeśli masz własną integrację z FastComments, możesz wprowadzić tutaj rodzaj identyfikatora zamiast URL.

#### Poziomy bezpieczeństwa

Wymagając SSO, będziesz musiał zdecydować, czy wymagasz Simple SSO czy Secure SSO. Jeśli wybierzesz Simple SSO, oba są dozwolone, ale jeśli wybierzesz Secure SSO, treść musi być pobierana z ładunkiem Secure SSO, który jest haszowany przy użyciu Twojego klucza API, aby mogła być wyświetlona.

Opcja poziomu bezpieczeństwa pojawi się po wybraniu „Wymagaj SSO, aby wyświetlać komentarze” i działa tylko wtedy, gdy ta opcja jest włączona.

The <a href="#sso-simple">Automatyczne weryfikowanie komentarzy za pomocą Simple SSO i zezwalanie administratorom Simple SSO</a> settings are separate, and apply whether or not this option is on.

#### Ochrona wykraczająca poza odczyt

Włączenie tej opcji zabezpieczy stronę lub domenę przed komentowaniem, chyba że użytkownik jest zalogowany przez SSO.

#### Pułapki

Użytkownicy, którzy utworzyli komentarze przed integracją SSO, nie będą mogli ich zobaczyć, chyba że zalogują się poprzez Twoją integrację SSO.