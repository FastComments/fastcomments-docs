A `PollVote` to odpowiedź jednej osoby na ankietę. Liczby wyświetlane na samej ankiecie są synchronizowane z tymi rekordami, więc potrzebujesz ich tylko wtedy, gdy chcesz wiedzieć *kto* na co zagłosował, a nie sumaryczne wyniki.

Głosujący może mieć maksymalnie jeden głos na ankietę. Ponowne głosowanie przenosi jego istniejący głos na nową opcję zamiast dodawać drugi, a `updatedAt` rejestruje, kiedy to nastąpiło.

`voterId` jest `userId`, gdy głosujący był zalogowany, w przeciwnym razie jest to `anonUserId`.

[inline-code-attrs-start title = 'Struktura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** The userId when the voter was logged in, otherwise the anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** When the voter last moved their vote to a different option. **/
    updatedAt?: string
}
[inline-code-end]

### Prywatność

Ustawienie `privacy` ankiety ma tak samo zastosowanie do tego API, jak ma w widżecie komentarzy:

- **Anonymous** (domyślnie): nikt nie może zobaczyć, jak ktoś zagłosował, więc głosy nie mogą być odczytane.  
  `GET /api/v1/poll-votes` i `GET /api/v1/poll-votes/:id` zwracają `poll-anonymous`. Liczby ankiety są nadal dostępne pod `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: Twój klucz API należy do administratora Twojej witryny, więc może odczytywać głosy.
- **Everyone**: głosy mogą być odczytane.

Prywatność ankiety może być ograniczona, ale nie może być rozszerzona po tym, jak pojawią się głosy.