Un oggetto `FeedPost` rappresenta un post in un feed di FastComments. Un feed è un flusso di post con i propri thread di commenti, renderizzato dal widget Feed. Ogni post ha un autore, contenuto ricco opzionale, media e link, e può essere taggato in modo che un feed possa essere filtrato.

La struttura dell'oggetto `FeedPost` è la seguente:

[inline-code-attrs-start title = 'Struttura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** L'ID dell'utente FastComments o SSO che ha scritto il post. **/
    fromUserId?: string
    /** Compilato dall'utente se non impostato. **/
    fromUserDisplayName?: string | null
    /** READONLY. Compilato dall'utente. **/
    fromUserAvatar?: string | null
    /** Usato per filtrare un feed. **/
    tags?: string[]
    /** Peso di ordinamento all'interno di un feed. Valori più alti vengono ordinati per primi. **/
    weight?: number
    /** Coppie chiave/valore libere per il tuo uso. **/
    meta?: Record<string, string>
    /** HTML sanitizzato. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaction type to count. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Dove il media punta quando viene cliccato. **/
    linkUrl?: string
    /** Una voce per ogni versione. Il widget sceglie la più adatta. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Il testo del link, ad esempio "Iscriviti ora". **/
    text?: string
    /** Un'intestazione mostrata con il link. **/
    title?: string
    /** Una descrizione mostrata con il link. **/
    description?: string
    url?: string
}
[inline-code-end]

Note:

- Alcuni di questi campi sono contrassegnati come `READONLY` - vengono restituiti dall'API ma non possono essere impostati.
- I commenti su un post sono commenti regolari il cui `urlId` è `post:` seguito dall'`_id` del post. Usa quel valore con l'API Commenti per leggere o creare commenti su un post.