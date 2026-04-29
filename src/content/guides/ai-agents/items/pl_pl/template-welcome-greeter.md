**ID szablonu:** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### Wbudowany początkowy komunikat

[inline-code-attrs-start title = 'Początkowy komunikat szablonu Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Wyzwalacze

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

This event fires exactly once per user, so the agent cannot loop. See [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Dozwolone narzędzia

- [`write_comment`](#tools-overview)

That is the only tool - the agent literally cannot moderate, vote, ban, or DM.

### Zalecane dodatki przed uruchomieniem na żywo

- **Ustaw nazwę wyświetlaną** na coś zachęcającego - "Community Bot", twoja maskotka strony lub nazwa twojej marki. Nazwa wyświetlana to to, co czytelnicy zobaczą przy odpowiedzi powitalnej.
- **Zaznacz "Include page title, subtitle, description, and meta tags"** w [Context Options](#context-options). Odpowiedzi powitalne stają się zauważalnie lepsze, gdy mogą odwołać się do tego, o czym naprawdę jest strona.
- **Rozważ ograniczenia dotyczące lokalizacji** jeśli działasz w wielu językach. Powitalna odpowiedź w złym języku jest bardziej jarring niż brak odpowiedzi. See [Scope: URL and Locale Filters](#scope-url-locale).

### Dlaczego zatwierdzenia nie są potrzebne

Agent tylko pisze nowe komentarze i tylko przy jednorazowym wyzwalaczu. Najgorszy scenariusz: niezręczne powitanie. Nie ma działania destrukcyjnego, które trzeba by zablokować. Większość operatorów uruchamia ten szablon bez żadnych zatwierdzeń, gdy dry-run wygląda czysto.

---