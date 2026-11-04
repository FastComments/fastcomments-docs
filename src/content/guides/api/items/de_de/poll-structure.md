Ein `Poll` ist an einen Kommentar angehängt, statt ein eigenständiges Objekt zu sein. Er wird mit dem Kommentar erstellt (siehe `POST /api/v1/comments`) oder später mit `PUT /api/v1/polls/:commentId` zu einem bestehenden Kommentar hinzugefügt.

Die Stimmenzahlen werden direkt am Poll gespeichert, sodass das Lesen eines Polls die Ergebnisse liefert, ohne dass etwas addiert werden muss. Die einzelnen Stimmen hinter diesen Zahlen sind `PollVote`-Objekte.

Jede Option hat eine `id`, die bei der Erstellung des Polls generiert wird. Diese id verwenden Sie, um eine Stimme abzugeben, eine Option umzubenennen und eine Option (samt ihrer Stimmen) beizubehalten, wenn Sie den Poll per `PUT` mit hinzugefügten oder entfernten Optionen aktualisieren. Es ist die einzige sichere Möglichkeit, auf eine Option zu verweisen - niemals ihre Position in der Liste.

[inline-code-attrs-start title = 'Poll-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Wenn gesetzt und in der Vergangenheit, ist der Poll geschlossen und akzeptiert keine Stimmen mehr. **/
    closesAt?: string | null
    /** 0 anonym (der Standard), 1 Admins und Moderatoren, 2 alle. Abwesend bedeutet anonym. **/
    privacy?: 0 | 1 | 2 | null
    /** Wenn true, sind die Zahlen für alle verborgen, die noch nicht abgestimmt haben. Abwesend bedeutet false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- Eine Frage ist erforderlich und hat maximal 200 Zeichen.
- Ein Poll hat zwischen 2 und 10 Optionen.
- Ein Optionslabel ist erforderlich, hat maximal 100 Zeichen und muss innerhalb des Polls eindeutig sein (Groß-/Kleinschreibung wird ignoriert).
- `closesAt` muss bei der Erstellung des Polls in der Zukunft liegen. Um einen Poll sofort zu schließen, führen Sie ein `PATCH` mit einem Datum in der Vergangenheit durch.

### Seiteneinstellungen

Polls folgen Ihrer Website-Konfiguration, die Sie unter Customize Widget ändern können:

- Polls müssen aktiviert sein, bevor ein Poll erstellt werden kann, andernfalls antwortet die API mit `polls-disabled`.
- Die Abstimmung kann auf angemeldete Benutzer beschränkt werden; in diesem Fall wird eine Stimme, die nur mit einer `anonUserId` gesendet wird, mit `poll-login-required` abgelehnt.