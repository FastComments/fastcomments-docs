Wyzwalane, gdy moderator oznacza komentarz jako spam.

### Kontekst otrzymywany przez agenta

- Komentarz, z flagą post-akcji `Is Spam`.
- **ID użytkownika wywołującego** - moderator, który podjął działanie.
- Opcjonalna historia wątku / użytkownika / kontekst strony zgodnie z konfiguracją.

### Kto to wyzwala

Akcja wykonana przez ludzkiego moderatora. Oznaczenia spamu pochodzące od agenta (przez [`mark_comment_spam`](#tools-overview)) **nie** wywołują tego wyzwalacza.

### Typowe zastosowania

- **Zapisywanie pamięci** - niech agent zapisze notatkę pamięciową o użytkowniku oznaczonym jako spam (np. "wcześniej oznaczony jako spam za X przez moderatora"), aby przyszłe agenty moderujące miały kontekst.
- **Egzekwowanie na poziomie użytkownika** - oznaczenie komentarza jako spam przez moderatora może być sygnałem dla agenta do wystawienia ostrzeżenia lub krótkiego bana, pod warunkiem zatwierdzenia.
- **Lustrzane odzwierciedlenie w systemie zewnętrznym** poprzez [Webhooks](#webhooks-overview).

---