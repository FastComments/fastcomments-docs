Na planie Flex osoby, które mogą administrować lub moderować Twoje konto, są rozliczane jako miesięczne stanowiska (seats). To ta część rachunku Flex, która najczęściej zaskakuje użytkowników, zwykle dlatego, że stanowiska zostały utworzone bez czyjejkolwiek wiedzy.

### Co Liczy Się Jako Stanowisko

- **Administratorzy**: użytkownicy na stronie `Users` Twojego konta z jakimkolwiek uprawnieniem administracyjnym (rozliczenia, dostosowywanie, analityka, moderacja itd.). Pierwszy administrator, zwykle właściciel konta, jest darmowy; każdy kolejny administrator to jedno stanowisko.
- **Moderatorzy**: osoby na stronie `Moderators`, które zaakceptowały swoje zaproszenie. Oczekujące zaproszenie nie jest rozliczane. Użytkownik, którego jedynym uprawnieniem jest moderacja komentarzy, jest rozliczany jako moderator, a nie administrator.
- **Administratorzy SSO** i **Moderatorzy SSO**: użytkownicy SSO, których Twoja witryna loguje z ustawionymi flagami `isAccountOwner`, `isAdminAdmin` lub `isCommentModeratorAdmin` w payloadzie SSO. Są rozliczani według tych samych stawek co zwykli administratorzy i moderatorzy. Zobacz [Struktura użytkownika SSO](/guide-api.html#sso-user-structure).

Stanowiska są deduplikowane według adresu e-mail, więc jedna osoba, która jest jednocześnie administratorem i moderatorem, lub która ma zarówno użytkownika SSO, jak i login do panelu z tym samym adresem e-mail, jest rozliczana raz, według wyższej stawki. Osoba z dwoma różnymi adresami e-mail to dwa stanowiska.

Na kontach rozliczanych według miesięcznie aktywnych użytkowników moderator SSO, który nie zalogował się w danym miesiącu, nie jest rozliczany za ten miesiąc.

### Stanowiska, Których Nie Dodałeś

Dwie funkcje tworzą stanowiska automatycznie:

- **Synchronizacja administratorów SSO.** Gdy użytkownik SSO loguje się z adresem e-mail pasującym do jednego z użytkowników Twojego panelu, uprawnienia administratora lub moderatora tego użytkownika są kopiowane do użytkownika SSO. W przypadku SSO WordPress oznacza to, że każdy administrator WordPress może stać się administratorem lub moderatorem FastComments. Wyłącz to w sekcji `Customize` opcją `Disable Automatic Admin Migration` w swojej regule dostosowywania.
- **Różne adresy e-mail tej samej osoby.** Jeśli Twój login do panelu używa jednego adresu e-mail, a użytkownik SSO innego, to są dwa stanowiska. Użycie tego samego adresu e-mail w obu miejscach je scala.

### Sprawdzanie I Usuwanie Stanowisk

Strona [Analityka rozliczeń](https://fastcomments.com/auth/my-account/analytics/billing) wyświetla liczby `Moderators`, `SSO Admins` i `SSO Moderators`, a podgląd następnej faktury pokazuje, ile kosztują. Usuń niepotrzebne osoby ze stron `Users` i `Moderators` albo usuń flagi administratora z payloadu SSO. Stanowiska są liczone przy tworzeniu faktury na koniec miesiąca, więc stanowisko usunięte przed tym momentem nie jest rozliczane za dany miesiąc.

Zobacz [Dodawanie moderatorów](/guide-users.html#adding-moderators) i [Aktualizacja uprawnień administratora](/guide-users.html#updating-administrator-permissions), aby dowiedzieć się, jak zarządzać oboma.