**Template ID:** `gaslight_detector`

Detektor Gaslight obserwuje edycje komentarzy, które przepisują historię w trakcie rozmowy – takie, w których autor zmienia znaczenie wcześniejszego komentarza po tym, jak zostały napisane odpowiedzi, powodując, że dalsze odpowiedzi wydają się nie na miejscu lub błędne. Gdy agent uzna, że edycja przekracza tę granicę, przywraca oryginalny tekst i wysyła DM do autora z wyjaśnieniem.

Jest to szablon o wyższym ryzyku, ponieważ modyfikuje treść użytkownika. Uruchamiaj go w trybie [dry-run](#dry-run-mode) dłużej niż szablon tylko do odczytu i ogranicz `edit_comment` za pomocą [approval](#approval-workflow), dopóki nie będziesz ufać ocenie modelu w odniesieniu do Twojego ruchu.

### Wyzwalacze

- **Komentarz edytowany** (`COMMENT_EDIT`) – agent porównuje nowy i poprzedni tekst i decyduje, czy edycja zniekształca istniejące już odpowiedzi.

Zobacz [Trigger: Comment Edited](#trigger-comment-edit), aby uzyskać pełny ładunek, w tym poprzedni tekst komentarza oraz liczbę odpowiedzi w momencie edycji.

### Dozwolone narzędzia

- [`edit_comment`](#tool-edit-comment) – używany do przywrócenia oryginalnego tekstu, gdy edycja zostanie uznana za gaslighting.
- [`warn_user`](#tool-warn-user) – wydaje łagodne ostrzeżenie, które użytkownik zobaczy przy następnym odwiedzeniu.
- [`send_dm`](#tools-overview) – kanał wyjaśniający; użytkownik otrzymuje wiadomość prywatną opisującą, dlaczego jego edycja została cofnięta.

Nie może banować, oznaczać spamu, głosować ani publikować nowych komentarzy – interfejs jest celowo wąski.

### Zalecane dodatki przed uruchomieniem

- **Ogranicz `edit_comment` za pomocą [approval](#approval-workflow).** Cofnięcie komentarza jest widoczne dla autora i dla każdego, kto widział edytowaną wersję, więc fałszywy pozytyw może być kłopotliwy. Trzymaj zatwierdzenia włączone, dopóki tryb dry-run nie pokaże, że agent jest konsekwentny.
- **Uściśnij prompt, określając, co na Twojej stronie jest uznawane za gaslighting.** Domyślny prompt jest celowo krótki. Dostarcz modelowi konkretne przykłady – „odwrócenie twierdzenia tak/nie”, „usunięcie liczby, na którą odwołują się odpowiedzi”, „dodanie wrogiego zdania po opublikowaniu odpowiedzi” – oraz wyraźne przeciwprzykłady, takie jak poprawki literówek, czyszczenie formatowania czy dodawanie źródeł.
- **Użyj liczby odpowiedzi z kontekstu wyzwalacza.** Edycje komentarzy bez odpowiedzi nie mogą zakłócić rozmowy; prompt powinien nakazać modelowi pomijanie takich przypadków.
- **Zaznacz „Include commenter's trust factor, account age, ban history, and recent comments”** w [Context Options](#context-options). Model jest znacznie mniej agresywny, gdy widzi konto o długiej, dobrej reputacji.
- **Rozważ krótkie okno tolerancji na edycję w promptcie.** Wiele edycji w ciągu pierwszych 30–60 sekund to poprawki literówek; poinstruuj model, aby ignorował tak szybkie edycje.

### Zalecane okno trybu dry-run

Uruchom przynajmniej przez dwa tygodnie rzeczywistego ruchu w trybie [dry-run](#dry-run-mode) przed przełączeniem na Włączone i przeglądaj każdą oznaczoną edycję w tym okresie. Skorzystaj z [Test Runs (Replays)](#test-runs-replays), aby odtworzyć ostatnie 30 dni edycji przeciwko agentowi przed uruchomieniem.