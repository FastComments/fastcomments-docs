Wyzwala agenta za każdym razem, gdy na stronie objętej [zakresem](#scope-url-locale) agenta zostanie opublikowany nowy komentarz.

### Kontekst, który otrzymuje agent

- Nowy komentarz w całości - tekst, autor, głosy, ID rodzica, ID URL strony.
- Opcjonalnie: komentarz nadrzędny i wcześniejsze odpowiedzi w tym samym wątku, jeśli włączony jest [kontekst wątku](#context-options).
- Opcjonalnie: współczynnik zaufania komentującego, wiek konta, historia banów i ostatnie komentarze, jeśli włączony jest [kontekst historii użytkownika](#context-options).
- Opcjonalnie: metadane strony, jeśli włączony jest [kontekst strony](#context-options).

### Ważne

- Wyzwalacz uruchamia się **po** zapisaniu komentarza. Agent może odwołać się do niego bezpośrednio w wywołaniach narzędzi.
- Nie uruchamia się **dla** komentarzy napisanych przez innego agenta w tym samym tenant.
- Uruchamia się zarówno dla zweryfikowanych, jak i niezweryfikowanych komentarzy. Jeśli twój tenant wymaga zatwierdzenia przez moderatora zanim komentarz będzie widoczny (zobacz [How Approvals Work](/guide-moderation.html#moderation-approvals) w przewodniku moderacji), wyzwalacz uruchamia się w momencie utworzenia komentarza, a nie wtedy, gdy zostanie później zatwierdzony. Bota moderatora można poinstruować, aby po przeglądzie zatwierdzał komentarze w twoim imieniu.

### Typowe zastosowania

- **Moderacja** - sprawdź komentarz pod kątem wytycznych społeczności, oznacz spam lub ostrzeż nowych użytkowników.
- **Powitanie** - chociaż [Wyzwalacz: Pierwszy komentarz nowego użytkownika](#trigger-new-user-first-comment) zwykle lepiej nadaje się do powitań, ponieważ uruchamia się raz na użytkownika.
- **Podsumowanie wątku** - zwykle łączy się z [opóźnieniem wyzwalacza](#trigger-deferred-delay), aby wątek się ustabilizował zanim agent zostanie uruchomiony.