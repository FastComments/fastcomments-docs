---
Można blokować użytkowników korzystających z określonych dostawców poczty e‑mail przy użyciu znaków wieloznacznych.

Na przykład, jeśli zauważysz, że wszystkie komentarze od **@bademail.com** są spamem, możesz po prostu zablokować całego dostawcę poczty, wpisując "*@bademail.com" w polu wprowadzania adresu e‑mail przy dodawaniu zablokowanego użytkownika.

Zwróć uwagę na "*" przed @ w adresie e‑mail.

### Subdomains

Blokada domeny obejmuje również wszystkie poddomeny tej domeny. Zablokowanie `*@bademail.com` blokuje także `someone@mail.bademail.com` oraz `someone@eu.mail.bademail.com`, więc nie ma potrzeby dodawania osobnej blokady dla każdej poddomeny.

Jeśli chcesz zablokować tylko konkretną poddomenę, wprowadź tę poddomenę, na przykład `*@mail.bademail.com`. Taka blokada nie wpływa na `someone@bademail.com`.

### Banning a Domain From a Comment

Nie musisz wpisywać wzorca ręcznie. Gdy blokujesz użytkownika z komentarza na stronie Moderate Comments, okno dialogowe blokady zawiera pole wyboru "Ban All @domain Users", które tworzy taką samą blokadę `*@domain` dla domeny e‑mail komentującego.

### Supported Patterns

Jedyną obsługiwaną formą wieloznacznika jest pojedynczy `*` zamiast całej części nazwy, po którym następuje `@` i domena. Inne formy są odrzucane, gdy próbujesz je zapisać:

- `*@*.bademail.com` nie jest potrzebny, ponieważ `*@bademail.com` już obejmuje poddomeny.
- `name*@bademail.com` oraz `*bademail.com` nie są obsługiwane.
---