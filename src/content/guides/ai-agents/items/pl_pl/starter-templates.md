FastComments dostarcza cztery szablony startowe, dzięki czemu nie musisz tworzyć działającego agenta od zera. Do nich można uzyskać dostęp ze [Strony Agentów AI](https://fastcomments.com/auth/my-account/ai-agents) klikając **Przeglądaj szablony**.

When you pick a template:

1. The agent is created with **Status: Dry Run** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). If that name is taken on your tenant, a numeric suffix is added.
2. Zostaniesz bezpośrednio przeniesiony do formularza edycji z wszystkim wstępnie wypełnionym - promptem, wyzwalaczami, dozwolonymi działaniami i ewentualnymi progami. Baner u góry informuje: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nic jeszcze nie jest włączone. Agent nie podejmie działań, dopóki nie zapiszesz i nie pozostawisz trybu symulacji włączonego (aby obserwować), lub nie przełączysz na Włączony.

### The four templates

- **[Moderator](#template-moderator)** - przegląda nowe i zgłoszone komentarze, ostrzega użytkowników naruszających zasady po raz pierwszy, eskaluje do zablokowania dopiero po ostrzeżeniu. Aktywuje się przy nowych komentarzach i przy przekroczeniu progu zgłoszeń (domyślny próg zgłoszeń: 3). Dozwolone narzędzia: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - odpowiada serdecznie i krótko nowym komentującym przy ich pierwszym komentarzu. Aktywuje się przy new-user-first-comment. Dozwolone narzędzie: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - przypina merytoryczne komentarze najwyższego poziomu, gdy przekroczą próg głosów (domyślny próg: 10), najpierw odpinając wcześniej przypięty komentarz. Aktywuje się przy przekroczeniu progu głosów. Dozwolone narzędzia: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publikuje neutralne, jednoparagrafowe podsumowanie długich wątków po opóźnieniu, a następnie je przypina. Aktywuje się przy nowych komentarzach z 30-minutowym opóźnieniem, aby wątek się ustabilizował przed podsumowaniem. Dozwolone narzędzia: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

Templates are starting points, not contracts. You are expected to:

- Dostosuj **Initial prompt** tak, aby pasował do stylu wypowiedzi twojej społeczności.
- Dodaj lub usuń **Triggers**, aby dopasować częstotliwość działania agenta.
- Dodaj **Approvals** dla każdej wrażliwej akcji — zdecydowanie zalecamy umieszczenie `ban_user` za zatwierdzeniem w szablonach moderatorów.
- Dodaj **Community guidelines**, aby agent stosował waszą pisaną politykę konsekwentnie. Zobacz [Community Guidelines](#community-guidelines).
- Ustaw per-agenta **Budgets** odpowiednie do liczby wyzwalaczy, których oczekujesz.

Szablon to tylko narzędzie, które wypełnia sensowne domyślne ustawienia; po zapisaniu agent jest twój.

---