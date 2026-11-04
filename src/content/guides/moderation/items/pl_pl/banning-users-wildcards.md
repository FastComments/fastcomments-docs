Można blokować użytkowników korzystających z określonych dostawców poczty e-mail, używając symboli wieloznacznych.

Na przykład, jeśli stwierdzisz, że wszystkie komentarze z **@bademail.com** to spam, możesz po prostu zablokować
całego tego dostawcę poczty, wpisując "*@bademail.com" w polu adresu e-mail podczas dodawania zablokowanego użytkownika.

Zwróć uwagę na "*" przed @ w adresie e-mail.

### Subdomeny

Blokada domeny obejmuje również każdą subdomenę tej domeny. Zablokowanie `*@bademail.com` blokuje także
`someone@mail.bademail.com` i `someone@eu.mail.bademail.com`, więc nie ma potrzeby dodawania osobnej blokady dla każdej subdomeny.

Jeśli chcesz zablokować tylko konkretną subdomenę, wpisz tę subdomenę, na przykład `*@mail.bademail.com`. Taka blokada
nie wpływa na `someone@bademail.com`.

### Blokowanie domeny z poziomu komentarza

Nie musisz samodzielnie wpisywać wzorca. Gdy blokujesz użytkownika z poziomu komentarza na stronie Moderate Comments, okno dialogowe blokady
zawiera pole wyboru "Ban All @domain Users", które tworzy taką samą blokadę `*@domain` dla domeny e-mail komentującego.

### Obsługiwane wzorce

Jedynym obsługiwanym formatem symbolu wieloznacznego jest pojedynczy `*` w miejsce całej części nazwy, po którym następuje `@` i domena. Inne formy
są odrzucane przy próbie zapisania:

- `*@*.bademail.com` nie jest potrzebne, ponieważ `*@bademail.com` już obejmuje subdomeny.
- `name*@bademail.com` i `*bademail.com` nie są obsługiwane.