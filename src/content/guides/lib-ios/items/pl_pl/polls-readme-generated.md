Ankiety działają tak samo jak w widżecie webowym i włączają się z tych samych ustawień widżetu w panelu FastComments: `pollCreation` (kto może dołączyć ankietę: wyłączone, administratorzy i moderatorzy, lub wszyscy) oraz `pollVoting` (zalogowani użytkownicy lub ktokolwiek).

- Komentarze z ankietą pokazują ją pod swoim tekstem. Czytelnicy głosują, dotykając opcji, mogą pokazać lub ukryć wyniki przed głosowaniem i mogą zmienić swój głos. Wyniki aktualizują się na żywo, gdy inni głosują.
- Ankiety mogą zamykać się w określonym dniu, ukrywać wyniki do momentu oddania głosu oraz pokazywać głosujących administratorom lub wszystkim („Zobacz, kto głosował").
- Gdy widz może tworzyć ankiety, pole wprowadzania komentarza w `FastCommentsView` pokazuje przycisk ankiety, który otwiera kreator ankiety. Ankieta jest publikowana razem z komentarzem, więc tekst komentarza jest wymagany. Pole wprowadzania czatu na żywo nie zawiera tego przycisku, podobnie jak czat na żywo w wersji webowej.

Głosowanie wymaga zalogowanego użytkownika, więc użyj [SSO](#authentication-sso-readme-generated). Serwer wiże anonimowy głos z plikiem cookie sesji, którego klient API nie przechowuje, więc przy `pollVoting` ustawionym na „ktokolwiek", widz bez użytkownika SSO zobaczy „Zaloguj się, aby głosować". Ankiety ukrywające wyniki do momentu głosowania nie otrzymują aktualizacji wyników na żywo; ci głosujący zobaczą nowe liczby przy następnym załadowaniu.

Aby opublikować ankietę z własnego interfejsu, przekaż `CommentPollInput` do `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` i `sdk.getPollVoters(commentId:page:search:)` obsługują głosowanie i listę głosujących. Kolory ankiet są opisane w sekcji [Kolory ankiet](#theming-readme-generated).