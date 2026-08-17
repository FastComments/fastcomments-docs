[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Questa API è usata per ottenere i commenti da visualizzare a un utente. Ad esempio, filtra automaticamente i commenti non approvati o spam.

### Pagination

Pagination può essere eseguita in uno dei due modi, a seconda dei requisiti di prestazioni e del caso d'uso:

1. **Più veloce:** **Precalculated Pagination**:
   1. Questo è il modo in cui FastComments funziona quando utilizzi i nostri widget e client predefiniti.
   2. Cliccare "next" aumenta semplicemente il conteggio delle pagine.
   3. Puoi considerarlo come recuperato da un archivio chiave-valore.
   4. In questo modo, definisci semplicemente un parametro `page` che parte da `0` e una direzione di ordinamento come `direction`.
   5. Le dimensioni delle pagine possono essere personalizzate tramite regole di personalizzazione.
2. **Più flessibile:** **Flexible Pagination**:
   1. In questo modo puoi definire parametri personalizzati `limit` e `skip`. Non passare `page`.
   2. Anche la `direction` di ordinamento è supportata.
   3. `limit` è il numero totale da restituire dopo l'applicazione di `skip`.
      - Esempio: imposta `skip = 200, limit = 100` quando `page size = 100` e `page = 2`.
   4. I commenti figli contano ancora nella paginazione. Puoi aggirare questo usando l'opzione `asTree`.
      - Puoi paginare i figli tramite `limitChildren` e `skipChildren`.
      - Puoi limitare la profondità dei thread restituiti tramite `maxTreeDepth`.

### Threads

1. Quando si utilizza `Precalculated Pagination`, i commenti sono raggruppati per *page* e i commenti nei thread influenzano l'intera pagina.
   1. In questo modo, i thread possono essere determinati sul client in base a `parentId`.
   2. Ad esempio, con una pagina con un commento di livello superiore e 29 risposte, e impostando `page=0` nell'API - otterrai solo il commento di livello superiore e i 29 figli.
2. Quando si utilizza `Flexible Pagination`, è possibile definire un parametro `parentId`.
   1. Impostalo a null per ottenere solo i commenti di livello superiore.
   2. Poi, per visualizzare i thread, chiama nuovamente l'API e passa `parentId`.
   3. Una soluzione comune è effettuare una chiamata API per i commenti di livello superiore e poi chiamate API parallele per ottenere i commenti dei figli di ciascun commento.
3. __NEW A partire da Feb 2023!__ Recupera come albero usando `&asTree=true`.
   1. Puoi considerarlo come `Flexible Pagination as a Tree`.
   2. Solo i commenti di livello superiore contano nella paginazione.
   3. Imposta `parentId=null` per avviare l'albero alla radice (devi impostare `parentId`).
   4. Imposta `skip` e `limit` per la paginazione.
   5. Imposta `asTree` a `true`.
   6. Il costo in crediti aumenta di `2x`, poiché il nostro backend deve fare molto più lavoro in questo scenario.
   7. Imposta `maxTreeDepth`, `limitChildren` e `skipChildren` come desiderato.

### Trees Explained

Quando si utilizza `asTree`, può essere difficile ragionare sulla paginazione. Ecco un grafico utile:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramma della paginazione ad albero" />
</div>

### Fetching Comments in The Context of a User

L'API `/comments` può essere usata in due contesti, per diversi casi d'uso:

- Per restituire i commenti ordinati e etichettati con informazioni per costruire il tuo client.
  - In questo caso, definisci un parametro di query `contextUserId`.
- Per recuperare i commenti dal tuo backend per integrazioni personalizzate.
  - La piattaforma predefinirà questo senza `contextUserId`. 

[inline-code-attrs-start title = 'Commenti Pagina Precalcolata'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Commenti Pagina Flessibile'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Commenti Pagina Flessibile nel Contesto Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Commenti Pagina Flessibile nel Contesto Utente solo per Commenti di Livello Superiore'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

È possibile ottenere i commenti restituiti come albero, con la paginazione che conta solo i commenti di livello superiore.

[inline-code-attrs-start title = 'Commenti Come-Albero nel Contesto Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vuoi ottenere solo i commenti di livello superiore e i figli immediati? Ecco un modo:

[inline-code-attrs-start title = 'Commenti Come-Albero con Profondità Massima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Tuttavia, nella tua UI potresti aver bisogno di sapere se mostrare un pulsante "mostra risposte" su ogni commento. Quando si recuperano i commenti tramite un albero, c'è una proprietà `hasChildren` etichettata sui commenti quando applicabile.

### Get Comments as a Tree, Searching by Hash Tag

È possibile cercare per hashtag usando l'API, su tutto il tuo tenant (non limitato a una singola pagina, o `urlId`).

In questo esempio, omettiamo `urlId` e cerchiamo per più hashtag. L'API restituirà solo i commenti che hanno tutti gli hashtag richiesti.

[inline-code-attrs-start title = 'Commenti Come-Albero nel Contesto Utente, per Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Struttura Richiesta Commenti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### The Response

[inline-code-attrs-start title = 'Struttura Risposta Commenti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Helpful Tips

#### URL ID

Probabilmente vuoi usare l'API `Comment` con il parametro `urlId`. Puoi chiamare prima l'API `Pages`, per vedere come appaiono i valori `urlId` disponibili per te. 

#### Anonymous Actions

Per i commenti anonimi probabilmente vuoi passare `anonUserId` quando recuperi i commenti, e quando esegui segnalazioni e blocchi.

(!) Questo è richiesto per molti store di app poiché gli utenti devono poter segnalare contenuti creati dagli utenti che possono vedere, anche se non hanno effettuato l'accesso. Non farlo può causare la rimozione della tua app da tale store.

#### Comments Not Being Returned

Verifica che i tuoi commenti siano approvati e non siano spam.

---