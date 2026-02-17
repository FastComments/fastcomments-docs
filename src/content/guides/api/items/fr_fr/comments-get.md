[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Cette API est utilisée pour obtenir des commentaires à afficher à un utilisateur. Par exemple, elle filtre automatiquement
les commentaires non approuvés ou spam.

### Pagination

La pagination peut être effectuée de deux manières, selon les exigences de performance et le cas d'utilisation :

1. Le plus rapide : **Pagination Précalculée** :
   1. C'est ainsi que FastComments fonctionne lorsque vous utilisez nos widgets et clients préconçus.
   2. Cliquer sur "suivant" augmente simplement le nombre de pages.
   3. Vous pouvez penser à cela comme étant récupéré par un magasin clé-valeur.
   4. De cette façon, définissez simplement un paramètre `page` commençant à `0` et une direction de tri comme `direction`.
   5. Les tailles de page peuvent être personnalisées via les règles de personnalisation.
2. Le plus flexible : **Pagination Flexible** :
   1. De cette façon, vous pouvez définir des paramètres `limit` et `skip` personnalisés. Ne passez pas `page`.
   2. La `direction` de tri est également supportée.
   3. `limit` est le total à retourner après que `skip` soit appliqué.
      - Exemple : définir `skip = 200, limit = 100` quand `taille de page = 100` et `page = 2`.
   4. Les commentaires enfants comptent toujours dans la pagination. Vous pouvez contourner cela en utilisant l'option `asTree`.
      - Vous pouvez paginer les enfants via `limitChildren` et `skipChildren`.
      - Vous pouvez limiter la profondeur des fils retournés via `maxTreeDepth`.

### Fils de discussion

1. Lors de l'utilisation de la `Pagination Précalculée`, les commentaires sont regroupés par *page* et les commentaires dans les fils affectent la page globale.
   1. De cette façon, les fils peuvent être déterminés côté client basé sur `parentId`.
   2. Par exemple, avec une page avec un commentaire de premier niveau, et 29 réponses, et en définissant `page=0` dans l'API - vous obtiendrez juste le commentaire de premier niveau et les 29 enfants.
   3. [Image d'exemple ici illustrant plusieurs pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Lors de l'utilisation de la `Pagination Flexible`, vous pouvez définir un paramètre `parentId`.
   1. Définissez-le à null pour obtenir uniquement les commentaires de premier niveau.
   2. Ensuite pour voir les fils, appelez à nouveau l'API et passez `parentId`.
   3. Une solution commune est de faire un appel API pour les commentaires de premier niveau puis de faire des appels API parallèles pour obtenir les commentaires pour les enfants de chaque commentaire.
3. __NOUVEAU Depuis Fév 2023!__ Récupérer sous forme d'arbre en utilisant `&asTree=true`.
   1. Vous pouvez penser à cela comme `Pagination Flexible sous forme d'Arbre`.
   2. Seuls les commentaires de premier niveau comptent dans la pagination.
   3. Définissez `parentId=null` pour démarrer l'arbre à la racine (vous devez définir `parentId`).
   4. Définissez `skip` et `limit` pour la pagination.
   5. Définissez `asTree` à `true`.
   6. Le coût en crédits augmente de `2x`, car notre backend doit faire beaucoup plus de travail dans ce scénario.
   7. Définissez `maxTreeDepth`, `limitChildren`, et `skipChildren` selon vos besoins.

### Les Arbres Expliqués

Lors de l'utilisation de `asTree`, il peut être difficile de raisonner sur la pagination. Voici un graphique pratique :

<div class="screenshot white-bg">
    <div class="title">Diagramme de Pagination en Arbre</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramme de Pagination en Arbre" />
</div>

### Récupérer des Commentaires dans le Contexte d'un Utilisateur

L'API `/comments` peut être utilisée dans deux contextes, pour différents cas d'utilisation :

- Pour retourner des commentaires triés et tagués avec des informations pour construire votre propre client.
  - Dans ce cas, définissez un paramètre de requête `contextUserId`.
- Pour récupérer des commentaires depuis votre backend pour des intégrations personnalisées.
  - La plateforme utilisera par défaut ceci sans `contextUserId`.

[inline-code-attrs-start title = 'Commentaires avec Pagination Précalculée'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires avec Pagination Flexible'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires avec Pagination Flexible dans le Contexte Utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires avec Pagination Flexible dans le Contexte Utilisateur pour les Commentaires de Premier Niveau Uniquement'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Obtenir des Commentaires sous forme d'Arbre

Il est possible d'obtenir les commentaires retournés sous forme d'arbre, avec la pagination ne comptant que les commentaires de premier niveau.

[inline-code-attrs-start title = 'Commentaires sous forme d’Arbre dans le Contexte Utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vous voulez obtenir uniquement les commentaires de premier niveau et les enfants immédiats ? Voici une façon :

[inline-code-attrs-start title = 'Commentaires sous forme d’Arbre avec Profondeur Maximale'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Cependant, dans votre interface vous pourriez avoir besoin de savoir s'il faut afficher un bouton "afficher les réponses" sur
chaque commentaire. Lors de la récupération des commentaires via un arbre, il y a une propriété `hasChildren` taguée
sur les commentaires lorsque applicable.

### Obtenir des Commentaires sous forme d'Arbre, en Recherchant par Hash Tag

Il est possible de rechercher par hashtag en utilisant l'API, à travers tout votre locataire (pas limité à une page, ou `urlId`).

Dans cet exemple, nous omettons `urlId`, et nous recherchons par plusieurs hashtags. L'API ne retournera que les commentaires qui ont tous les hashtags demandés.

[inline-code-attrs-start title = 'Commentaires sous forme d’Arbre dans le Contexte Utilisateur, par Hash Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Tous les Paramètres de Requête

[inline-code-attrs-start title = 'Structure de Requête des Commentaires'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### La Réponse

[inline-code-attrs-start title = 'Structure de Réponse des Commentaires'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Conseils Utiles

#### URL ID

Vous voudrez probablement utiliser l'API `Comment` avec le paramètre `urlId`. Vous pouvez appeler l'API `Pages` d'abord, pour voir à quoi ressemblent les valeurs `urlId` disponibles pour vous.

#### Actions Anonymes

Pour les commentaires anonymes, vous voudrez probablement passer `anonUserId` lors de la récupération des commentaires, et lors du signalement et du blocage.

(!) Ceci est requis pour de nombreux magasins d'applications car les utilisateurs doivent pouvoir signaler le contenu créé par les utilisateurs qu'ils peuvent voir, même s'ils ne sont pas connectés. Ne pas le faire peut causer le retrait de votre application dudit magasin.

#### Commentaires Non Retournés

Vérifiez que vos commentaires sont approuvés, et ne sont pas spam.
