**Template ID:** `gaslight_detector`

Detektor gaslightingu monitoruje edycje komentarzy, które przepisują historię w trakcie rozmowy — tego rodzaju, gdy autor zmienia znaczenie wcześniejszego komentarza po tym, jak pojawiły się odpowiedzi, przez co późniejsze odpowiedzi wyglądają na wyrwane z kontekstu lub błędne. Gdy agent uzna, że edycja przekroczyła tę granicę, przywraca oryginalny tekst i wysyła autorowi wiadomość prywatną z wyjaśnieniem.

To szablon o wyższym ryzyku, ponieważ modyfikuje treść użytkownika. Uruchom go w [dry-run](#dry-run-mode) dłużej niż szablon tylko do odczytu i umieść `edit_comment` za [approval](#approval-workflow), dopóki nie zaufasz ocenie modelu dla twojego ruchu.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - agent porównuje nowy i poprzedni tekst i decyduje, czy edycja wypacza istniejące odpowiedzi.

See [Trigger: Comment Edited](#trigger-comment-edit) for the full payload, including the previous comment text and reply count at edit time.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - używane do przywrócenia oryginalnego tekstu, gdy edycja zostanie oceniona jako gaslighting.
- [`warn_user`](#tool-warn-user) - wydaje miękkie ostrzeżenie, które użytkownik zobaczy przy następnej wizycie.
- [`send_dm`](#tools-overview) - kanał wyjaśnień; użytkownik otrzymuje wiadomość prywatną opisującą, dlaczego jego edycja została cofnięta.

It cannot ban, mark spam, vote, or post new comments - the surface is intentionally narrow.

### Recommended additions before going live

- **Gate `edit_comment` behind [approval](#approval-workflow).** Przywrócenie komentarza jest widoczne dla autora i dla każdego, kto widział edytowaną wersję, więc fałszywy pozytyw jest kompromitujący. Trzymaj zatwierdzenia włączone, dopóki [dry-run](#dry-run-mode) nie pokaże, że agent działa konsekwentnie.
- **Tighten the prompt with what counts as gaslighting on your site.** Domyślny prompt jest krótki celowo. Podaj modelowi konkretne przykłady — „odwrócenie twierdzenia tak/nie”, „usunięcie liczby, na którą powołują się odpowiedzi”, „dodanie wrogiego zdania po opublikowaniu odpowiedzi” — oraz wyraźne przykłady negatywne, takie jak poprawki literówek, sprzątanie formatowania lub dodawanie źródeł.
- **Use the reply count from the trigger context.** Edycje komentarzy bez odpowiedzi nie mogą wypaczyć rozmowy; prompt powinien nakazać modelowi pomijać takie przypadki.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). Model jest znacznie mniej agresywny, gdy może zobaczyć konto działające w dobrej wierze od dłuższego czasu.
- **Consider a short edit-grace window in the prompt.** Wiele edycji w ciągu pierwszych 30–60 sekund to poprawki literówek; poinstruuj model, aby ignorował tak szybkie edycje.

### Recommended dry-run window

Run for at least two weeks of real traffic in [dry-run](#dry-run-mode) before flipping to Enabled, and review every flagged edit during that window. Use [Test Runs (Replays)](#test-runs-replays) to replay the last 30 days of edits against the agent before going live.