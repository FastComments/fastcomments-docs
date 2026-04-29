Wywoływane, gdy użytkownik publikuje swój pierwszy komentarz na tej stronie (your tenant). To jest **raz na użytkownika** - kolejne komentarze tego samego użytkownika nie spowodują ponownego wywołania.

### Kontekst, który otrzymuje agent

- Nowy komentarz.
- Opcjonalny kontekst wątku / historii użytkownika / strony zgodnie z konfiguracją.

Gdy kontekst historii użytkownika jest włączony, lista ostatnich komentarzy użytkownika oczywiście będzie pusta (lub będzie zawierać tylko ten jeden), ale współczynnik zaufania i wiek konta zostaną uzupełnione.

### Ważne

- "First comment on this site" odnosi się do zakresu **tenant**, a nie do wszystkich stron FastComments. Użytkownik mający komentarze na innych stronach FastComments nadal wywoła ten trigger przy pierwszym opublikowaniu komentarza na Twojej stronie.
- Wyzwalacz uruchamia się tylko dla użytkowników posiadających userId. Anonimowe, niezweryfikowane komentarze bez stabilnego userId nie powodują jego wyzwolenia.
- Wyzwalacz uruchamia się, gdy komentarz zostanie zatwierdzony/widoczny (nie w momencie początkowego opublikowania). Edycje lub działania moderatora dotyczące pierwszych komentarzy nie powodują ponownego uruchomienia.

### Typowe zastosowania

- **Powitanie** - szablon [Welcome Greeter template](#template-welcome-greeter) jest zbudowany wokół tego wyzwalacza.
- **Onboarding** - wyślij [DM warning](#tool-warn-user) (używane tutaj jako przyjazne powiadomienie, a nie ostrzeżenie) wskazujące użytkownikowi zasady społeczności.
- **Powiadomienie recenzenta** - jeśli chcesz, żeby człowiek obejrzał pierwszy wpis każdego nowego komentującego, [`mark_comment_reviewed`](#tools-overview) może oznaczyć je do przeglądu.