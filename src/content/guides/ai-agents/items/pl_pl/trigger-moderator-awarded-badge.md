Wyzwalane, gdy moderator przyznaje użytkownikowi odznakę.

### Kontekst, który otrzymuje agent

- The **badge ID** of the badge awarded.
- The **triggering user ID** - the moderator who awarded the badge.
- Opcjonalny kontekst wątku / historii użytkownika / strony zgodnie z konfiguracją.

Źródło wywołania nie zawiera `commentId` w ładunku wyzwalacza, nawet jeśli odznaka została przyznana w odniesieniu do konkretnego komentarza.

### Kto wyzwala to

Ręczna akcja moderatora.

### Ważne

- The badge ID alone is included; the agent does not receive the badge metadata (name, image). Jeśli agent musi rozpoznać, *która* odznaka została przyznana, osadź ten kontekst w [początkowym poleceniu](#personality-prompt) lub w [wytycznych społeczności](#community-guidelines).
- Wyzwalacz uruchamia się raz na każde przyznanie odznaki, nie raz na użytkownika. Przyznanie tej samej odznaki temu samemu użytkownikowi dwukrotnie spowoduje dwukrotne wyzwolenie (każde przyznanie to odrębne zdarzenie).

### Typowe zastosowania

- **Reciprocal recognition** - agent może opublikować odpowiedź „dziękujemy za wspaniały wkład”, gdy przyznana zostanie określona odznaka.
- **External recognition workflow** via [Webhooki](#webhooks-overview) - odzwierciedlaj przyznania odznak w swoim systemie zaangażowania użytkowników.
- **Memory recording** - notatki „ten użytkownik jest uznanym współtwórcą”, które przyszłe agenty moderujące powinny brać pod uwagę w swoich decyzjach.