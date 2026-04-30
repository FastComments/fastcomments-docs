FastComments dostarcza pięć szablonów startowych, abyś nie musiał pisać działającego agenta od podstaw. Można do nich przejść ze [strony Agentów AI](https://fastcomments.com/auth/my-account/ai-agents) klikając **Przeglądaj szablony**.

When you pick a template:

1. The agent is created with **Status: Tryb próbny** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). If that name is taken on your tenant, a numeric suffix is added.
2. You land directly on the edit form with everything pre-filled - prompt, triggers, allowed actions, and any thresholds. A banner across the top reads "Utworzono na podstawie szablonu {templateName}. Przejrzyj ustawienia poniżej, a gdy będziesz gotowy, zmień status na Włączony."
3. Nothing is enabled yet. The agent will not act until you save and either keep dry-run on (to observe) or flip to Enabled.

### Pięć szablonów

- **[Moderator](#template-moderator)** - przegląda nowe i oznaczone komentarze, ostrzega użytkowników przy pierwszym przewinieniu, eskaluje do zbanowania dopiero po ostrzeżeniu. Uruchamia się przy nowych komentarzach oraz gdy przekroczony zostanie próg oznaczeń (domyślny próg oznaczeń: 3). Dozwolone narzędzia: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - odpowiada serdecznie pierwszym komentującym krótkim, osobistym powitaniem. Uruchamia się przy pierwszym komentarzu nowego użytkownika. Dozwolone narzędzie: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - przypina merytoryczne komentarze najwyższego poziomu, gdy przekroczą próg głosów (domyślnie: 10), najpierw odpinając poprzednio przypięty komentarz. Uruchamia się przy przekroczeniu progu głosów. Dozwolone narzędzia: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publikuje neutralne, jednoakapitowe podsumowanie długich wątków po opóźnieniu, a następnie je przypina. Uruchamia się przy nowych komentarzach z 30-minutowym opóźnieniem, aby wątek się ustabilizował przed podsumowaniem. Dozwolone narzędzia: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - monitoruje edycje komentarzy pod kątem modyfikacji w środku wątku, które wypaczają odpowiedzi, przywraca oryginalny tekst i wysyła wiadomość prywatną do autora. Uruchamia się przy edycjach komentarzy. Dozwolone narzędzia: `edit_comment`, `warn_user`, `send_dm`.

### Dostosowywanie szablonu

Szablony to punkty wyjścia, a nie zobowiązania. Oczekuje się, że:

- Dostosujesz **Początkowy prompt** do tonu twojej społeczności.
- Dodasz lub usuniesz **Wyzwalacze**, aby dopasować, jak często agent ma się uruchamiać.
- Dodasz **Zatwierdzenia** dla każdej wrażliwej akcji — zdecydowanie zalecamy zabezpieczenie `ban_user` za pomocą zatwierdzenia w szablonach typu moderator.
- Dodasz **Zasady społeczności**, aby agent stosował twoją pisemną politykę konsekwentnie. Zobacz [Zasady społeczności](#community-guidelines).
- Ustawisz dla każdego agenta **Budżety** odpowiednie do oczekiwanej liczby wyzwalaczy.

Szablon to tylko narzędzie, które wstępnie uzupełnia sensowne wartości domyślne; po zapisaniu agent należy do ciebie.