FastComments SSO (<a href="#sso">szczegóły tutaj</a>) zapewnia Twoim użytkownikom możliwość komentowania bez konieczności logowania się do innej platformy.

Jednak samo to nie zabezpiecza wątków komentarzy, ponieważ domyślnie dane komentarzy są informacją publiczną - każdy, kto może zobaczyć stronę, może zobaczyć komentarze.

Zmieniając ustawienie, możemy zabronić pobierania komentarzy, chyba że robi to administrator lub prawidłowy użytkownik SSO.

#### Konfiguracja bez kodu

Możemy uniemożliwić przeglądanie i interakcję z naszymi wątkami komentarzy, gdy SSO jest skonfigurowane, tworząc <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regułę dostosowywania</a>.

When doing so, search for SSO, and you will find this option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Włącz tę opcję i zapisz regułę dostosowywania.

#### Ochrona wybranej domeny lub strony

Aby chronić tylko określoną Domain lub Page, po prostu skonfigurujemy regułę dostosowywania tak, aby to robiła.

Na górze interfejsu dostosowywania znajdziemy dwa pola wejściowe, Domain i URL ID.

Aby chronić konkretną domenę, wpisz daną domenę w pole "domain".

Aby chronić konkretną stronę, wpisz adres URL strony w polu "URL ID". Jeśli masz niestandardową integrację z FastComments, możesz zamiast URL wpisać tutaj inny typ identyfikatora.

#### Poziomy zabezpieczeń

Decydując się na wymaganie SSO, będziesz chciał zdecydować, czy wymagasz Simple SSO czy Secure SSO. Jeśli wymagasz Simple SSO, obie metody są dozwolone, ale jeśli wymagasz Secure SSO, to zawartość musi być pobrana z Secure SSO payload zahaszowanym Twoim API key, aby mogła być wyświetlona.

Opcja poziomu zabezpieczeń pojawi się, gdy wybierzesz "Require SSO To View Comments".

#### Ochrona wykraczająca poza odczyt

Włączenie tej opcji zabezpieczy stronę lub domenę przed dodawaniem komentarzy, chyba że użytkownik jest zalogowany przez SSO.

#### Uwaga

Użytkownicy, którzy utworzyli komentarze przed integracją SSO, nie będą mogli ich zobaczyć, chyba że zalogują się przez Twoją integrację SSO.