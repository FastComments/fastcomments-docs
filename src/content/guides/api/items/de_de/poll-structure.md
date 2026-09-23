Ein `Poll` ist an einen Kommentar angehängt, anstatt ein eigenständiges Objekt zu sein. Es wird zusammen mit dem Kommentar erstellt (siehe `POST /api/v1/comments`), oder später zu einem bestehenden Kommentar hinzugefügt mit `PUT /api/v1/polls/:commentId`.

Die Stimmenzahlen werden direkt in der Umfrage gespeichert, sodass das Auslesen einer Umfrage Ihnen die Ergebnisse liefert, ohne dass Sie etwas zusammenzählen müssen. Die einzelnen Stimmen hinter diesen Zahlen sind `PollVote`‑Objekte.

Jede Option hat eine `id`, die beim Erstellen der Umfrage generiert wird. Diese `id` verwenden Sie, um eine Stimme abzugeben, eine Option umzubenennen und eine Option (und ihre Stimmen) beizubehalten, wenn Sie die Umfrage mit `PUT` aktualisieren und Optionen hinzufügen oder entfernen. Sie ist die einzige sichere Möglichkeit, sich auf eine Option zu beziehen – niemals auf ihre Position in der Liste.

[inline-code-attrs-start title = 'Umfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- Eine Frage ist erforderlich und darf höchstens 200 Zeichen lang sein.
- Eine Umfrage hat zwischen 2 und 10 Optionen.
- Eine Optionsbezeichnung ist erforderlich, darf höchstens 100 Zeichen lang sein und muss innerhalb der Umfrage eindeutig sein (Groß‑/Kleinschreibung wird ignoriert).
- `closesAt` muss beim Erstellen der Umfrage in der Zukunft liegen. Um eine Umfrage sofort zu schließen, führen Sie ein `PATCH` mit einem Datum in der Vergangenheit aus.

### Site Settings

Umfragen folgen Ihrer Seitenkonfiguration, die Sie unter **Customize Widget** ändern können:

- Umfragen müssen aktiviert sein, bevor eine Umfrage erstellt werden kann, sonst antwortet die API mit `polls-disabled`.
- Die Abstimmung kann auf angemeldete Benutzer beschränkt werden; in diesem Fall wird eine Stimme, die nur mit einer `anonUserId` gesendet wird, mit `poll-login-required` abgelehnt.