A `Poll` is attached to a comment, rather than being an object of its own. It is created with the comment
(see `POST /api/v1/comments`), or added to an existing comment later with `PUT /api/v1/polls/:commentId`.

The vote counts are kept on the poll itself, so reading a poll gives you the results without having to add
anything up. The individual votes behind those counts are `PollVote` objects.

Each option has an `id` that is generated when the poll is created. That id is what you use to cast a vote,
to relabel an option, and to keep an option (and its votes) when you `PUT` the poll with options added or
removed. It is the only safe way to refer to an option - never its position in the list.

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

### Limits

- Pytanie jest wymagane i może mieć maksymalnie 200 znaków.
- Ankieta ma od 2 do 10 opcji.
- Etykieta opcji jest wymagana, może mieć maksymalnie 100 znaków i musi być unikalna w ankiecie (bez uwzględniania wielkości liter).
- `closesAt` musi być w przyszłości w momencie tworzenia ankiety. Aby zamknąć ankietę natychmiast, wykonaj `PATCH` z datą w przeszłości.

### Site Settings

Polls obey your site configuration, which you can change under Customize Widget:

- Ankiety muszą być włączone, zanim będzie można utworzyć ankietę, w przeciwnym razie API zwróci `polls-disabled`.
- Głosowanie może być ograniczone do zalogowanych użytkowników; w takim przypadku głos wysłany jedynie z `anonUserId` zostanie odrzucony z kodem `poll-login-required`.