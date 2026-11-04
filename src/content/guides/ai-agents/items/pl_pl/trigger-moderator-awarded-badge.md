Uruchamia się, gdy moderator przyznaje odznakę użytkownikowi.

### Kontekst otrzymywany przez agenta

- **Identyfikator odznaki** przyznanej odznaki.
- **Identyfikator użytkownika wywołującego** - moderator, który przyznał odznakę.
- Opcjonalny kontekst wątku / historii użytkownika / strony, zgodnie z konfiguracją.

Zdarzenie **nie** zawiera `commentId` w ładunku wyzwalacza, nawet jeśli odznaka została przyznana w odniesieniu do konkretnego komentarza.

### Kto to uruchamia

Działanie moderatora (człowieka).

### Istotne informacje

- Zawarty jest sam identyfikator odznaki; agent nie otrzymuje metadanych odznaki (nazwa, obraz). Jeśli agent musi rozumować o tym, *która* odznaka została przyznana, osadź ten kontekst w [początkowym promptcie](#personality-prompt) lub [wytycznych społeczności](#community-guidelines).
- Wyzwalacz uruchamia się raz na przyznanie odznaki, a nie na użytkownika. Przyznanie tej samej odznaki użytkownikowi dwukrotnie uruchamia go dwukrotnie (każde przyznanie to odrębne zdarzenie).

### Typowe zastosowania

- **Wzajemne uznanie** - agent może opublikować odpowiedź "dziękuję za świetny wkład", gdy zostanie przyznana konkretna odznaka.
- **Zewnętrzny przepływ uznania** za pośrednictwem [Webhooks](#webhooks-overview) - odzwierciedlaj przyznawanie odznak we własnym systemie zaangażowania użytkowników.
- **Zapisywanie w pamięci** - notatki "ten użytkownik jest uznanym współtwórcą", które przyszłe agenty moderacji powinny uwzględniać w swoich decyzjach.