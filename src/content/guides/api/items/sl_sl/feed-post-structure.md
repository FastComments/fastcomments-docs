A `FeedPost` objekt predstavlja objavo v FastComments viru. Vir je tok objav s svojimi komentarji, prikazan z gradnikom Feed. Vsaka objava ima avtorja, izbirno bogato vsebino, medije in povezave ter jo je mogoče označiti, da se lahko vir filtrira.

Struktura objekta `FeedPost` je naslednja:

[inline-code-attrs-start title = 'Struktura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** ID FastComments ali SSO uporabnika, ki je napisal objavo. **/
    fromUserId?: string
    /** Izpolnjeno iz uporabnika, ko ni nastavljeno. **/
    fromUserDisplayName?: string | null
    /** READONLY. Izpolnjeno iz uporabnika. **/
    fromUserAvatar?: string | null
    /** Uporabljeno za filtriranje vira. **/
    tags?: string[]
    /** Utež za razvrščanje v viru. Višje vrednosti se razvrstijo najprej. **/
    weight?: number
    /** Pari ključ/vrednost v prosti obliki za lastno uporabo. **/
    meta?: Record<string, string>
    /** Sanitiziran HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Vrsta reakcije za štetje. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Kam se medijski element poveže ob kliku. **/
    linkUrl?: string
    /** En vnos na vsako izvedbo. Gradnik izbere najboljšo možnost. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Besedilo povezave, npr. "Prijavite se zdaj". **/
    text?: string
    /** Naslov, prikazan s povezavo. **/
    title?: string
    /** Opis, prikazan s povezavo. **/
    description?: string
    url?: string
}
[inline-code-end]

- Nekatera od teh polj so označena z `READONLY` – ta so vrnjena s strani API, vendar jih ni mogoče nastaviti.
- Komentarji na objavi so običajni komentarji, katerih `urlId` je `post:` sledi ID objave `_id`. To vrednost uporabite z API-jem za komentarje, da preberete ali ustvarite komentarje na objavi.