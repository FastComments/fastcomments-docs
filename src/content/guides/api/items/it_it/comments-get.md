[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Questa API viene utilizzata per ottenere i commenti da mostrare a un utente. Per esempio, filtra automaticamente
i commenti non approvati o lo spam.

### Paginazione

La paginazione può essere fatta in uno di due modi, a seconda dei requisiti di performance e del caso d'uso:

1. Più veloce: **Paginazione Precalcolata**:
   1. Questo è il modo in cui FastComments funziona quando si utilizzano i nostri widget e client predefiniti.
   2. Cliccare su "next" semplicemente incrementa il conteggio della pagina.
   3. Si può pensare a questo come recuperato da un key-value store.
   4. In questo modo, definire semplicemente un parametro `page` che inizia da `0` e una direzione di ordinamento `direction`.
   5. Le dimensioni delle pagine possono essere personalizzate tramite regole di customizzazione.
2. Più flessibile: **Paginazione Flessibile**:
   1. In questo modo puoi definire parametri personalizzati `limit` e `skip`. Non passare `page`.
   2. È supportata anche la direzione di ordinamento `direction`.
   3. `limit` è il numero totale da restituire dopo che `skip` è stato applicato.
      - Esempio: imposta `skip = 200, limit = 100` quando `page size = 100` e `page = 2`.
   4. I commenti figli contano ancora nella paginazione. Puoi aggirare questo usando l'opzione `asTree`.
      - Puoi effettuare la paginazione dei figli tramite `limitChildren` e `skipChildren`.
      - Puoi limitare la profondità dei thread restituiti tramite `maxTreeDepth`.

### Thread

1. Quando si utilizza la `Paginazione Precalcolata`, i commenti sono raggruppati per *pagina* e i commenti nei thread influenzano la pagina complessiva.
   1. In questo modo, i thread possono essere determinati sul client in base a `parentId`.
   2. Per esempio, con una pagina con un commento di primo livello e 29 risposte, impostando `page=0` nell'API — otterrai solo il commento di primo livello e i 29 figli.
   3. [Immagine di esempio qui che illustra più pagine.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Quando si utilizza la `Paginazione Flessibile`, puoi definire un parametro `parentId`.
   1. Impostalo su null per ottenere solo i commenti di primo livello.
   2. Poi, per visualizzare i thread, richiama l'API e passa `parentId`.
   3. Una soluzione comune è effettuare una chiamata API per i commenti di primo livello e poi effettuare chiamate API parallele per ottenere i commenti figli di ciascun commento.
3. __NUOVO da febbraio 2023!__ Recupera come albero usando `&asTree=true`.
   1. Puoi pensarlo come una `Paginazione Flessibile come Albero`.
   2. Solo i commenti di primo livello contano nella paginazione.
   3. Imposta `parentId=null` per iniziare l'albero alla radice (devi impostare `parentId`).
   4. Imposta `skip` e `limit` per la paginazione.
   5. Imposta `asTree` su `true`.
   6. Il costo in crediti aumenta di `2x`, poiché il nostro backend deve fare molto più lavoro in questo scenario.
   7. Imposta `maxTreeDepth`, `limitChildren` e `skipChildren` come desiderato.

### Alberi Spiegati

Quando si usa `asTree`, può essere difficile comprendere la paginazione. Ecco un grafico utile:

<div class="screenshot white-bg">
    <div class="title">Diagramma della Paginazione ad Albero</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramma della Paginazione ad Albero" />
</div>

### Recupero dei Commenti nel Contesto di un Utente

L'API `/comments` può essere utilizzata in due contesti, per diversi casi d'uso:

- Per restituire commenti ordinati e taggati con informazioni per costruire il proprio client.
  - In questo caso, definisci un parametro di query `contextUserId`.
- Per recuperare commenti dal tuo backend per integrazioni personalizzate.
  - La piattaforma userà questo comportamento di default senza `contextUserId`. 

[inline-code-attrs-start title = 'Paginazione Precalcolata dei Commenti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Paginazione Flessibile dei Commenti'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Paginazione Flessibile dei Commenti nel Contesto Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Paginazione Flessibile dei Commenti nel Contesto Utente per Solo Commenti di Primo Livello'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Ottenere i Commenti come un Albero

È possibile ottenere i commenti restituiti come un albero, con la paginazione che conta solo i commenti di primo livello.

[inline-code-attrs-start title = 'Commenti Come Albero nel Contesto Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vuoi ottenere solo i commenti di primo livello e i figli immediati? Ecco un modo:

[inline-code-attrs-start title = 'Commenti Come Albero con Profondità Massima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Tuttavia, nella tua UI potresti aver bisogno di sapere se mostrare un pulsante "mostra risposte" su
ogni commento. Quando si recuperano commenti tramite un albero esiste una proprietà `hasChildren` aggiunta
ai commenti quando applicabile.

### Ottenere i Commenti come un Albero, Ricerca per Hashtag

È possibile cercare per hashtag usando l'API, su tutto il tenant (non limitato a una singola pagina o a `urlId`).

In questo esempio, omettiamo `urlId`, e cerchiamo per più hashtag. L'API restituirà solo i commenti che contengono tutti gli hashtag richiesti.

[inline-code-attrs-start title = 'Commenti Come Albero nel Contesto Utente, per Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Tutti i Parametri della Richiesta

[inline-code-attrs-start title = 'Struttura della Richiesta dei Commenti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### La Risposta

[inline-code-attrs-start title = 'Struttura della Risposta dei Commenti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Suggerimenti Utili

#### URL ID

Probabilmente vorrai usare l'API `Comment` con il parametro `urlId`. Puoi chiamare prima l'API `Pages`, per vedere come sono i valori `urlId` disponibili per te. 

#### Azioni Anonime

Per commentare in modo anonimo probabilmente vorrai passare `anonUserId` quando recuperi i commenti, e quando effettui segnalazioni e blocchi.

(!) Questo è richiesto da molti store di app poiché gli utenti devono essere in grado di segnalare contenuti creati dagli utenti che possono vedere, anche se non hanno effettuato l'accesso. Non farlo potrebbe causare la rimozione della tua app da tali store.

#### Commenti Non Restituiti

Controlla che i tuoi commenti siano approvati e che non siano spam.

---