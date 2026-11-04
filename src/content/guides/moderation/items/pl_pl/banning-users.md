Istnieją dwa sposoby zablokowania użytkownikom możliwości komentowania na Twojej stronie za pomocą FastComments.

Pierwszy z nich: jeśli znasz już adres e-mail użytkownika, możesz go wprowadzić na stronie <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista zablokowanych użytkowników w sekcji Moderate Comments, z zablokowanymi adresami e-mail i przyciskiem dodawania nowej blokady'; title='Strona Banned Users' app-screenshot-end]

Strona ta jest dostępna poprzez Moderate Comments -> Banned Users

Gdy chcemy zablokować użytkownika, możemy wybrać typ blokady: Permanent lub Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Formularz nowej blokady z polem e-mail i wyborem typu blokady: Permanent lub Permanent Shadow Ban'; title='Blokowanie użytkownika' app-screenshot-end]

Drugim sposobem zablokowania użytkownika jest kliknięcie przycisku blokady, który znajduje się przy każdym komentarzu na stronie Comment Moderation.

Po kliknięciu przycisku blokady pojawią się opcje, w których można określić typ i czas trwania blokady.

### Aliasy e-mail

Podczas blokowania użytkownika po adresie e-mail FastComments automatycznie ignoruje aliasy z `+`. Na przykład zablokowanie `user+alias@gmail.com` zablokuje również `user@gmail.com` oraz wszelkie inne warianty z `+` tego adresu, takie jak `user+other@gmail.com`.

### Shadow Bany

Shadow ban to rodzaj blokady, przy którym wygląda na to, że komentarz lub głos użytkownika został pomyślnie zapisany, choć w rzeczywistości nie został. Może to być pożądane w pewnych sytuacjach.

### Blokowanie po adresie IP

O ile klient nie zdecyduje inaczej, FastComments obsługuje blokowanie po IP poprzez przechowywanie zahaszowanej wersji adresu IP komentującego.

### Wyszukiwanie zablokowanych użytkowników

Gdy lista urośnie do ponad jednej lub dwóch stron, możesz ją zawęzić za pomocą wiersza wyszukiwania nad tabelą.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Wiersz wyszukiwania na stronie Banned Users z listą rozwijaną Search By, listą rozwijaną Match i polem Value'; title='Wyszukiwanie zablokowanych użytkowników' app-screenshot-end]

Dostępne są trzy kontrolki:

- **Search By** wybiera pole, w którym szukać: Any Field, Email, Name, Banned By lub Banned For Saying. Ostatnie cztery odpowiadają kolumnom o tych samych nazwach w tabeli.
- **Match** wybiera sposób porównania. **Contains** znajduje wartość w dowolnym miejscu pola, a **Equals** dopasowuje całe pole.
- **Value** to tekst do wyszukania.

Każde pole jest dopasowywane bez rozróżniania wielkości liter, więc wyszukiwanie `SPAMMER@EXAMPLE.COM` znajdzie blokadę zapisaną jako `spammer@example.com`.

Kilka rzeczy warte uwagi:

- **Banned For Saying** przeszukuje treść komentarza, za który użytkownik został zablokowany. W ten sposób znajdziesz wszystkich zablokowanych za konkretne sformułowanie.
- **Banned By** przeszukuje nazwę moderatora, który nałożył blokadę, co jest przydatne przy weryfikacji decyzji innego moderatora.
- Blokady z symbolami wieloznacznymi są przechowywane ze swoim `*`, więc wyszukiwanie **Contains** dla `bademail.com` znajdzie blokadę `*@bademail.com`.
- **Name** dopasowuje nazwę wyświetlaną w kolumnie Name, więc znajdzie użytkownika nawet jeśli zmienił nazwę po zablokowaniu, a także jeśli blokadę utworzyłeś, podając tylko adres e-mail i nazwa nie została wtedy zapisana. Nazwa zapisana w blokadzie również nadal pasuje, więc wyszukiwanie działa zarówno dla starej, jak i aktualnej nazwy.
- **Any Field** przeszukuje jednocześnie e-mail, nazwę, moderatora blokującego oraz treść zablokowanego komentarza.

Twoje wyszukiwanie jest częścią adresu URL strony, więc możesz udostępnić przefiltrowaną listę innym moderatorom w ten sam sposób, w jaki udostępniasz inne linki moderacyjne. Przechodzenie między stronami wyników zachowuje wyszukiwanie, rozpoczęcie nowego wyszukiwania przenosi Cię na pierwszą stronę, a **Clear** przywraca pełną listę.