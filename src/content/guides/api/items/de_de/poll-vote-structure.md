Ein `PollVote` ist die Antwort einer Person auf eine Umfrage. Die auf der Umfrage selbst angezeigten Zähler werden mit diesen synchron gehalten, sodass Sie diese nur benötigen, wenn Sie wissen wollen *wer* für was gestimmt hat, anstatt die Gesamtsummen.

Ein Wähler hat höchstens eine Stimme pro Umfrage. Ein erneutes Abstimmen verschiebt seine bestehende Stimme zur neuen Option, anstatt eine zweite hinzuzufügen, und `updatedAt` zeichnet auf, wann das geschehen ist.

`voterId` ist die `userId`, wenn der Wähler eingeloggt war, und sonst die `anonUserId`.

[inline-code-attrs-start title = 'PollVote-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** Die userId, wenn der Wähler eingeloggt war, sonst die anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Wann der Wähler seine Stimme zuletzt zu einer anderen Option verschoben hat. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

Die `privacy`‑Einstellung der Umfrage gilt für diese API auf dieselbe Weise wie im Kommentar‑Widget:

- **Anonymous** (Standard): Niemand kann sehen, wie jemand abgestimmt hat, daher können die Stimmen nicht gelesen werden.  
  `GET /api/v1/poll-votes` und `GET /api/v1/poll-votes/:id` antworten mit `poll-anonymous`. Die Zähler der Umfrage sind weiterhin über `GET /api/v1/polls/:commentId` verfügbar.
- **Admins und Moderatoren**: Ihr API‑Schlüssel gehört dem Administrator Ihrer Seite, sodass er die Stimmen lesen kann.
- **Jeder**: Die Stimmen können gelesen werden.

Die Privatsphäre einer Umfrage kann nach dem ersten Stimmen abgegeben werden eingeschränkt, aber nicht erweitert werden.