FastComments SSO (<a href="#sso">szczegóły tutaj</a>) zapewnia Twoim użytkownikom możliwość komentowania bez konieczności logowania się na inną platformę.

Jednak samo to nie zabezpiecza wątków komentarzy, ponieważ domyślnie dane komentarzy są publicznie dostępne – każdy, kto może zobaczyć stronę, może zobaczyć komentarze.

Zmieniając ustawienie, możemy ograniczyć pobieranie komentarzy, chyba że zrobi to administrator lub ważny użytkownik SSO.

#### No-Code Setup

Możemy zapobiec przeglądaniu i interakcjom z naszymi wątkami komentarzy, gdy SSO jest skonfigurowane, tworząc <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regułę dostosowywania</a>.

Podczas tego, wyszukaj SSO, a znajdziesz tę opcję:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Włączona opcja wymogu SSO do wyświetlania komentarzy w regule dostosowywania, z wyborem poziomu bezpieczeństwa'; title='Wymagaj SSO, aby wyświetlać komentarze' app-screenshot-end]

Włącz ją i zapisz regułę dostosowywania.

#### Only Protect a Certain Domain or Page

Aby chronić tylko określoną domenę lub stronę, po prostu skonfigurujemy regułę dostosowywania, aby to zrobić.

U góry interfejsu dostosowywania znajdziemy dwa pola wejściowe: Domain i URL ID.

Aby chronić tylko konkretną domenę, wprowadź tę domenę w polu „domain”.

Aby chronić konkretną stronę, wprowadź URL strony w polu „URL ID”. Jeśli masz własną integrację z FastComments, możesz wprowadzić tutaj rodzaj identyfikatora zamiast URL.

#### Security Levels

Wymagając SSO, będziesz chciał zdecydować, czy wymagasz Simple SSO czy Secure SSO. Jeśli wymagasz Simple SSO, oba są dozwolone, ale jeśli wymagasz Secure SSO, treść musi być pobierana z ładunkiem Secure SSO, który jest haszowany przy użyciu Twojego klucza API, aby mogła być wyświetlona.

Opcja poziomu bezpieczeństwa pojawi się po wybraniu „Require SSO To View Comments”.

#### Protection Beyond Reading

Włączenie tej opcji zabezpieczy stronę lub domenę przed komentowaniem, chyba że użytkownik jest zalogowany przez SSO.

#### Gotchas

Użytkownicy, którzy utworzyli komentarze przed integracją SSO, nie będą mogli ich zobaczyć, chyba że zalogują się poprzez Twoją integrację SSO.