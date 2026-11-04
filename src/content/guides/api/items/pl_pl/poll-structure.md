`Poll` jest dołączony do komentarza, a nie jest samodzielnym obiektem. Jest tworzony razem z komentarzem
(zobacz `POST /api/v1/comments`) lub dodawany do istniejącego komentarza później za pomocą `PUT /api/v1/polls/:commentId`.

Liczniki głosów są przechowywane bezpośrednio w ankiecie, więc odczyt ankiety daje wyniki bez konieczności
czegokolwiek sumowania. Poszczególne głosy stojące za tymi licznikami to obiekty `PollVote`.

Każda opcja ma `id` generowane w momencie utworzenia ankiety. Tego id używa się do oddania głosu,
zmiany etykiety opcji oraz do zachowania opcji (i jej głosów), gdy wysyłasz `PUT` ankiety z dodanymi lub
usuniętymi opcjami. To jedyny bezpieczny sposób odwoływania się do opcji - nigdy jej pozycja na liście.

[inline-code-attrs-start title = 'Struktura ankiety'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limity

- Pytanie jest wymagane i ma maksymalnie 200 znaków.
- Ankieta ma od 2 do 10 opcji.
- Etykieta opcji jest wymagana, ma maksymalnie 100 znaków i musi być unikalna w obrębie ankiety (bez rozróżniania wielkości liter).
- `closesAt` musi być datą w przyszłości w momencie utworzenia ankiety. Aby natychmiast zamknąć ankietę, wykonaj `PATCH` z
  datą w przeszłości.

### Ustawienia witryny

Ankiety podlegają konfiguracji witryny, którą możesz zmienić w sekcji Customize Widget:

- Ankiety muszą być włączone, zanim będzie można utworzyć ankietę, w przeciwnym razie API odpowiada błędem `polls-disabled`.
- Głosowanie może być ograniczone do zalogowanych użytkowników - w takim przypadku głos wysłany tylko z `anonUserId` zostanie
  odrzucony z błędem `poll-login-required`.