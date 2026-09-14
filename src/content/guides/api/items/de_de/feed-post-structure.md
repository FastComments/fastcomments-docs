Ein `FeedPost`-Objekt stellt einen Beitrag in einem FastComments-Feed dar. Ein Feed ist ein Strom von Beiträgen mit eigenen Kommentar‑Threads, die vom Feed‑Widget gerendert werden. Jeder Beitrag hat einen Autor, optionalen Rich‑Content, Medien und Links und kann getaggt werden, sodass ein Feed gefiltert werden kann.

Die Struktur des `FeedPost`‑Objekts ist wie folgt:

[inline-code-attrs-start title = 'FeedPost Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Die ID des FastComments- oder SSO‑Benutzers, der den Beitrag verfasst hat. **/
    fromUserId?: string
    /** Vom Benutzer ausgefüllt, wenn nicht gesetzt. **/
    fromUserDisplayName?: string | null
    /** READONLY. Vom Benutzer ausgefüllt. **/
    fromUserAvatar?: string | null
    /** Wird verwendet, um einen Feed zu filtern. **/
    tags?: string[]
    /** Sortiergewicht innerhalb eines Feeds. Höhere Werte werden zuerst sortiert. **/
    weight?: number
    /** Freiform Schlüssel/Wert‑Paare für den eigenen Gebrauch. **/
    meta?: Record<string, string>
    /** Bereinigtes HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaktionstyp zum Zählen. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Wohin das Medienelement beim Klicken verlinkt. **/
    linkUrl?: string
    /** Ein Eintrag pro Variante. Das Widget wählt die beste Passung. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Der Linktext, z. B. „Jetzt anmelden“. **/
    text?: string
    /** Eine Überschrift, die zusammen mit dem Link angezeigt wird. **/
    title?: string
    /** Eine Beschreibung, die zusammen mit dem Link angezeigt wird. **/
    description?: string
    url?: string
}
[inline-code-end]

- Einige dieser Felder sind mit `READONLY` gekennzeichnet – sie werden von der API zurückgegeben, können aber nicht gesetzt werden.
- Die Kommentare zu einem Beitrag sind reguläre Kommentare, deren `urlId` `post:` gefolgt von der Beitrags‑`_id` ist. Verwenden Sie diesen Wert mit der Comment‑API, um Kommentare zu einem Beitrag zu lesen oder zu erstellen.