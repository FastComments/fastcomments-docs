[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Cette API est utilisée pour récupérer les commentaires afin de les afficher à un utilisateur. Par exemple, elle filtre automatiquement
les commentaires non approuvés ou indésirables (spam).

### Pagination

La pagination peut être effectuée de deux manières, selon les exigences de performance et le cas d'utilisation :

1. La plus rapide : **Pagination Précalculée** :
   1. C'est ainsi que fonctionne FastComments lorsque vous utilisez nos widgets et clients préconstruits.
   2. Cliquer sur « suivant » incrémente simplement le numéro de page.
   3. Vous pouvez considérer cela comme une récupération via un magasin clé-valeur.
   4. De cette manière, définissez simplement un paramètre `page` commençant à `0` et une direction de tri via `direction`.
   5. La taille des pages peut être personnalisée via des règles de personnalisation.
2. La plus flexible : **Pagination Flexible** :
   1. De cette manière, vous pouvez définir des paramètres `limit` et `skip` personnalisés. Ne passez pas `page`.
   2. La `direction` de tri est également prise en charge.
   3. `limit` est le nombre total à retourner après application de `skip`.
      - Exemple : définissez `skip = 200, limit = 100` lorsque `page size = 100` et `page = 2`.
   4. Les commentaires enfants comptent toujours dans la pagination. Vous pouvez contourner cela avec l'option `asTree`.
      - Vous pouvez paginer les enfants via `limitChildren` et `skipChildren`.
      - Vous pouvez limiter la profondeur des fils retournés via `maxTreeDepth`.

### Fils de discussion

1. Lors de l'utilisation de la `Precalculated Pagination`, les commentaires sont regroupés par *page* et les commentaires dans les fils affectent la page globale.
   1. De cette manière, les fils peuvent être déterminés côté client à partir de `parentId`.
   2. Par exemple, avec une page contenant un commentaire de premier niveau et 29 réponses, et en définissant `page=0` dans l'API - vous obtiendrez uniquement le commentaire de premier niveau et les 29 enfants.
2. Lors de l'utilisation de la `Flexible Pagination`, vous pouvez définir un paramètre `parentId`.
   1. Définissez-le à null pour obtenir uniquement les commentaires de premier niveau.
   2. Ensuite, pour visualiser les fils, appelez à nouveau l'API en passant `parentId`.
   3. Une solution courante consiste à effectuer un appel API pour les commentaires de premier niveau, puis à effectuer des appels API en parallèle pour récupérer les commentaires des enfants de chaque commentaire.
3. __NOUVEAU Depuis février 2023 !__ Récupérez sous forme d'arbre avec `&asTree=true`.
   1. Vous pouvez considérer cela comme la `Flexible Pagination as a Tree`.
   2. Seuls les commentaires de premier niveau comptent dans la pagination.
   3. Définissez `parentId=null` pour démarrer l'arbre à la racine (vous devez définir `parentId`).
   4. Définissez `skip` et `limit` pour la pagination.
   5. Définissez `asTree` à `true`.
   6. Le coût en crédits augmente de `2x`, car notre backend doit effectuer beaucoup plus de travail dans ce scénario.
   7. Définissez `maxTreeDepth`, `limitChildren` et `skipChildren` selon vos besoins.

### Explication des arbres

Lors de l'utilisation de `asTree`, il peut être difficile de raisonner sur la pagination. Voici un graphique pratique :

<div class="screenshot white-bg">
    <div class="title">Diagramme de pagination en arbre</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramme de pagination en arbre" />
</div>

### Récupération des commentaires dans le contexte d'un utilisateur

L'API `/comments` peut être utilisée dans deux contextes, pour différents cas d'utilisation :

- Pour retourner des commentaires triés et étiquetés avec des informations permettant de construire votre propre client.
  - Dans ce cas, définissez un paramètre de requête `contextUserId`.
- Pour récupérer des commentaires depuis votre backend pour des intégrations personnalisées.
  - La plateforme utilisera par défaut ce mode sans `contextUserId`. 

[inline-code-attrs-start title = 'Pagination précalculée des commentaires'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Pagination flexible des commentaires'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Pagination flexible des commentaires dans le contexte utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Pagination flexible des commentaires dans le contexte utilisateur, commentaires de premier niveau uniquement'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Obtenir les commentaires sous forme d'arbre

Il est possible d'obtenir les commentaires sous forme d'arbre, la pagination ne comptant que les commentaires de premier niveau.

[inline-code-attrs-start title = 'Commentaires sous forme d\'arbre dans le contexte utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vous souhaitez obtenir uniquement les commentaires de premier niveau et leurs enfants immédiats ? Voici une méthode :

[inline-code-attrs-start title = 'Commentaires sous forme d\'arbre avec profondeur maximale'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Cependant, dans votre interface utilisateur, vous pourriez avoir besoin de savoir s'il faut afficher un bouton « afficher les réponses » sur
chaque commentaire. Lors de la récupération des commentaires sous forme d'arbre, une propriété `hasChildren` est ajoutée
aux commentaires le cas échéant.

### Obtenir les commentaires sous forme d'arbre, recherche par hashtag

Il est possible de rechercher par hashtag via l'API, sur l'ensemble de votre tenant (pas limité à une seule page ou `urlId`).

Dans cet exemple, nous omettons `urlId` et nous recherchons par plusieurs hashtags. L'API ne retournera que les commentaires contenant tous les hashtags demandés.

[inline-code-attrs-start title = 'Commentaires sous forme d\'arbre dans le contexte utilisateur, par hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Tous les paramètres de requête

[inline-code-attrs-start title = 'Structure de la requête Comments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Le urlId (URL de la page ou identifiant de l'article) auquel les commentaires sont associés. **/
    urlId?: string
    /** Limiter les commentaires retournés à cet utilisateur. **/
    userId?: string
    /** Utilisez ceci pour rechercher par hashtag. Pour affiner à l'intersection de plusieurs hashtags, faites &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** La direction du tri. La valeur par défaut est MR (Most Relevant). Les autres options sont OF (Oldest First) et NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Pagination précalculée : la page à récupérer, à partir de 0. Passez -1 pour tous les commentaires (jusqu'à 250). **/
    page?: number
    /** Pagination flexible : combien de commentaires devons-nous retourner ? **/
    limit?: number
    /** Pagination flexible : combien de commentaires enfants devons-nous retourner pour chaque parent ? **/
    limitChildren?: number
    /** Pagination flexible : combien de commentaires devons-nous ignorer ? **/
    skip?: number
    /** Pagination flexible : combien de commentaires enfants devons-nous ignorer pour chaque parent ? **/
    skipChildren?: number
    /** Pour déterminer les commentaires bloqués et signalés. **/
    contextUserId?: string
    /** Pour déterminer les commentaires bloqués et signalés. **/
    anonUserId?: string
    /** Pour récupérer les commentaires enfants. **/
    parentId?: string
    /** Pour la récupération sous forme d'arbre. **/
    asTree?: boolean
    /** Jusqu'où dans l'arbre devons-nous retourner des données ? 0 ne retourne aucun enfant. 1 retourne les enfants immédiats, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### La réponse

[inline-code-attrs-start title = 'Structure de la réponse Comments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Inclus en cas d'échec. **/
    reason?: string
    /** Les commentaires ! **/
    comments: Comment[]
}
[inline-code-end]

### Conseils utiles

#### URL ID

Vous voudrez probablement utiliser l'API `Comment` avec le paramètre `urlId`. Vous pouvez d'abord appeler l'API `Pages`, pour voir à quoi ressemblent les valeurs `urlId` disponibles.

#### Actions anonymes

Pour les commentaires anonymes, vous voudrez probablement passer `anonUserId` lors de la récupération des commentaires, ainsi que lors du signalement et du blocage.

(!) Ceci est requis par de nombreuses boutiques d'applications, car les utilisateurs doivent pouvoir signaler le contenu créé par les utilisateurs qu'ils peuvent voir, même s'ils ne sont pas connectés. Ne pas le faire peut entraîner le retrait de votre application de ladite boutique.

#### Commentaires non retournés

Vérifiez que vos commentaires sont approuvés et ne sont pas du spam.