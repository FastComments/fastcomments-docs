[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Questa API viene utilizzata per ottenere i commenti da mostrare a un utente. Ad esempio, filtra automaticamente
i commenti non approvati o contrassegnati come spam.

### Paginazione

La paginazione può essere effettuata in uno dei due modi seguenti, a seconda dei requisiti di prestazioni e del caso d'uso:

1. Più veloce: **Paginazione Precalcolata**:
   1. È così che funziona FastComments quando si utilizzano i nostri widget e client predefiniti.
   2. Cliccare su "successivo" semplicemente incrementa il numero di pagina.
   3. Si può pensare a questo come a un recupero da un archivio chiave-valore.
   4. In questo modo, basta definire un parametro `page` a partire da `0` e una direzione di ordinamento come `direction`.
   5. Le dimensioni della pagina possono essere personalizzate tramite regole di personalizzazione.
2. Più flessibile: **Paginazione Flessibile**:
   1. In questo modo è possibile definire parametri `limit` e `skip` personalizzati. Non passare `page`.
   2. Anche la `direction` di ordinamento è supportata.
   3. `limit` è il numero totale da restituire dopo l'applicazione di `skip`.
      - Esempio: impostare `skip = 200, limit = 100` quando `page size = 100` e `page = 2`.
   4. I commenti figli contano comunque nella paginazione. È possibile aggirare questo problema utilizzando l'opzione `asTree`.
      - È possibile paginare i figli tramite `limitChildren` e `skipChildren`.
      - È possibile limitare la profondità dei thread restituiti tramite `maxTreeDepth`.

### Thread

1. Quando si utilizza la `Paginazione Precalcolata`, i commenti sono raggruppati per *pagina* e i commenti nei thread influiscono sulla pagina complessiva.
   1. In questo modo, i thread possono essere determinati sul client in base a `parentId`.
   2. Ad esempio, con una pagina contenente un commento di primo livello e 29 risposte, impostando `page=0` nell'API - si otterrà solo il commento di primo livello e i 29 figli.
2. Quando si utilizza la `Paginazione Flessibile`, è possibile definire un parametro `parentId`.
   1. Impostarlo su null per ottenere solo i commenti di primo livello.
   2. Quindi, per visualizzare i thread, chiamare nuovamente l'API passando `parentId`.
   3. Una soluzione comune è effettuare una chiamata API per i commenti di primo livello e poi effettuare chiamate API in parallelo per ottenere i commenti dei figli di ciascun commento.
3. __NOVITÀ Da Febbraio 2023!__ Recupero come albero utilizzando `&asTree=true`.
   1. Si può pensare a questo come a una `Paginazione Flessibile come Albero`.
   2. Solo i commenti di primo livello contano nella paginazione.
   3. Impostare `parentId=null` per iniziare l'albero dalla radice (è necessario impostare `parentId`).
   4. Impostare `skip` e `limit` per la paginazione.
   5. Impostare `asTree` su `true`.
   6. Il costo in crediti aumenta del `2x`, poiché il nostro backend deve fare molto più lavoro in questo scenario.
   7. Impostare `maxTreeDepth`, `limitChildren` e `skipChildren` come desiderato.

### Spiegazione degli Alberi

Quando si utilizza `asTree`, può essere difficile ragionare sulla paginazione. Ecco un grafico utile:

<div class="screenshot white-bg">
    <div class="title">Diagramma della Paginazione ad Albero</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramma della Paginazione ad Albero" />
</div>

### Recupero dei Commenti nel Contesto di un Utente

L'API `/comments` può essere utilizzata in due contesti, per diversi casi d'uso:

- Per restituire commenti ordinati e etichettati con informazioni per la creazione del proprio client.
  - In questo caso, definire un parametro di query `contextUserId`.
- Per recuperare commenti dal proprio backend per integrazioni personalizzate.
  - La piattaforma passerà per impostazione predefinita a questa modalità senza `contextUserId`.

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

[inline-code-attrs-start title = 'Paginazione Flessibile dei Commenti nel Contesto Utente Solo per Commenti di Primo Livello'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Ottenere i Commenti come Albero

È possibile ottenere i commenti restituiti come albero, con la paginazione che conta solo i commenti di primo livello.

[inline-code-attrs-start title = 'Commenti Come Albero nel Contesto Utente'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Si vogliono ottenere solo i commenti di primo livello e i figli immediati? Ecco un modo:

[inline-code-attrs-start title = 'Commenti Come Albero con Profondità Massima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Tuttavia, nella propria UI potrebbe essere necessario sapere se mostrare un pulsante "mostra risposte" su
ciascun commento. Quando si recuperano i commenti come albero, esiste una proprietà `hasChildren` aggiunta
ai commenti quando applicabile.

### Ottenere i Commenti come Albero, Cercando per Hashtag

È possibile cercare per hashtag utilizzando l'API, su tutto il proprio tenant (non limitato a una pagina o a un `urlId`).

In questo esempio, omettiamo `urlId` e cerchiamo per più hashtag. L'API restituirà solo i commenti che hanno tutti gli hashtag richiesti.

[inline-code-attrs-start title = 'Commenti Come Albero nel Contesto Utente, Per Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Probabilmente si desidera utilizzare l'API `Comment` con il parametro `urlId`. È possibile chiamare prima l'API `Pages`, per vedere come sono i valori `urlId` disponibili.

#### Azioni Anonime

Per i commenti anonimi probabilmente si desidera passare `anonUserId` durante il recupero dei commenti e quando si effettuano segnalazioni e blocchi.

(!) Questo è richiesto da molti app store poiché gli utenti devono poter segnalare i contenuti creati dagli utenti che possono vedere, anche se non sono registrati. Non farlo può causare la rimozione della propria app da tale store.

#### Commenti Non Restituiti

Verificare che i propri commenti siano approvati e non siano spam.