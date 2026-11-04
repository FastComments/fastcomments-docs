FastComments dostarcza pięć szablonów startowych, dzięki czemu nie musisz pisać działającego agenta od zera. Są one dostępne na [stronie AI Agents](https://fastcomments.com/auth/my-account/ai-agents) po kliknięciu **Browse templates**.

Gdy wybierzesz szablon:

1. Agent jest tworzony ze statusem **Status: Dry Run** oraz wewnętrzną nazwą opartą na szablonie (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Jeśli ta nazwa jest już zajęta w Twojej tenancie, dodawany jest sufiks numeryczny.
2. Trafiasz bezpośrednio na formularz edycji z wszystkimi wypełnionymi polami - prompt, wyzwalacze, dozwolone akcje oraz wszelkie progi. Baner na górze strony informuje: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nic nie jest jeszcze włączone. Agent nie podejmie żadnych działań, dopóki nie zapiszesz konfiguracji i nie pozostawisz trybu dry-run (aby obserwować) lub nie przełączysz na Enabled.

### Pięć szablonów

- **[Moderator](#template-moderator)** - przegląda nowe i oflagowane komentarze, ostrzega pierwszorazowych sprawców, eskaluje do bana dopiero po ostrzeżeniu. Wyzwalany przez nowe komentarze oraz przekroczenia progu flagowania (domyślny próg flagowania: 3). Dozwolone narzędzia: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - ciepło odpowiada pierwszorazowym komentującym krótkim, osobistym powitaniem. Wyzwalany przez new-user-first-comment. Dozwolone narzędzie: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - przypina wartościowe komentarze najwyższego poziomu po przekroczeniu progu głosów (domyślnie: 10), najpierw odpinając wcześniej przypięty komentarz. Wyzwalany przez przekroczenia progu głosów. Dozwolone narzędzia: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publikuje neutralne, jednoakapitowe podsumowanie na długich wątkach po opóźnieniu, a następnie je przypina. Wyzwalany przez nowe komentarze z 30-minutowym odroczeniem, aby wątek „osiadł" przed podsumowaniem. Dozwolone narzędzia: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - obserwuje edycje komentarzy pod kątem zmian w trakcie wątku, które wypaczają odpowiedzi, przywraca oryginalny tekst i wysyła wiadomość DM do autora. Wyzwalany przez edycje komentarzy. Dozwolone narzędzia: `edit_comment`, `warn_user`, `send_dm`.

### Dostosowywanie szablonu

Szablony to punkty wyjścia, a nie sztywne reguły. Oczekuje się, że:

- Dostosujesz **Initial prompt**, aby pasował do głosu Twojej społeczności.
- Dodasz lub usuniesz **Triggers**, aby dopasować częstotliwość działania agenta.
- Dodasz **Approvals** dla każdej wrażliwej akcji - zdecydowanie zalecamy zabezpieczenie `ban_user` zatwierdzeniem w przypadku szablonów typu moderator.
- Dodasz **Community guidelines**, aby agent konsekwentnie stosował Twoją spisaną politykę. Zobacz [Community Guidelines](#community-guidelines).
- Ustawisz **Budgets** dla poszczególnych agentów odpowiednio do oczekiwanej liczby wyzwalaczy.

Szablon to jedynie nośnik, który wypełnia rozsądne wartości domyślne; po zapisaniu agent należy do Ciebie.