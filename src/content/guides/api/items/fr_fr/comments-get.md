[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Cette API est utilisée pour récupérer les commentaires à afficher à un utilisateur. Par exemple, elle filtre automatiquement les commentaires non approuvés ou indésirables.

### Pagination

La pagination peut être effectuée de deux manières, selon les exigences de performance et le cas d’utilisation :

1. **Le plus rapide : Pagination Précalculée** :
   1. C’est ainsi que FastComments fonctionne lorsque vous utilisez nos widgets et clients préconstruits.
   2. Cliquer sur « suivant » augmente simplement le compteur de pages.
   3. Vous pouvez considérer cela comme récupéré par un magasin clé-valeur.
   4. De cette façon, définissez simplement un paramètre `page` commençant à `0` et une direction de tri comme `direction`.
   5. Les tailles de page peuvent être personnalisées via des règles de personnalisation.
2. **Le plus flexible : Pagination Flexible** :
   1. De cette façon, vous pouvez définir des paramètres personnalisés `limit` et `skip`. Ne passez pas `page`.
   2. Le `direction` de tri est également pris en charge.
   3. `limit` est le nombre total à renvoyer après l’application de `skip`.
      - Exemple : définissez `skip = 200, limit = 100` lorsque `page size = 100` et `page = 2`.
   4. Les commentaires enfants comptent toujours dans la pagination. Vous pouvez contourner cela en utilisant l’option `asTree`.
      - Vous pouvez paginer les enfants via `limitChildren` et `skipChildren`.
      - Vous pouvez limiter la profondeur des fils retournés via `maxTreeDepth`.

### Threads

1. Lorsque vous utilisez la `Pagination Précalculée`, les commentaires sont regroupés par *page* et les commentaires dans les fils affectent la page globale.
   1. De cette façon, les fils peuvent être déterminés côté client à partir de `parentId`.
   2. Par exemple, avec une page contenant un commentaire de niveau supérieur et 29 réponses, et en définissant `page=0` dans l’API – vous obtiendrez uniquement le commentaire de niveau supérieur et les 29 enfants.
2. Lorsque vous utilisez la `Pagination Flexible`, vous pouvez définir un paramètre `parentId`.
   1. Définissez-le à null pour ne récupérer que les commentaires de niveau supérieur.
   2. Ensuite, pour voir les fils, appelez à nouveau l’API en passant `parentId`.
   3. Une solution courante consiste à faire un appel API pour les commentaires de niveau supérieur, puis à faire des appels API parallèles pour récupérer les commentaires des enfants de chaque commentaire.
3. __NOUVEAU Depuis février 2023 !__ Récupérez sous forme d’arbre en utilisant `&asTree=true`.
   1. Vous pouvez considérer cela comme `Pagination Flexible sous forme d’Arbre`.
   2. Seuls les commentaires de niveau supérieur comptent dans la pagination.
   3. Définissez `parentId=null` pour démarrer l’arbre à la racine (vous devez définir `parentId`).
   4. Définissez `skip` et `limit` pour la pagination.
   5. Définissez `asTree` à `true`.
   6. Le coût en crédits augmente de `2x`, car notre backend doit effectuer beaucoup plus de travail dans ce scénario.
   7. Définissez `maxTreeDepth`, `limitChildren` et `skipChildren` selon vos besoins.

### Trees Explained

Lorsque vous utilisez `asTree`, il peut être difficile de raisonner sur la pagination. Voici un graphique pratique :

<div class="screenshot white-bg">
    <div class="title">Diagramme de Pagination d'Arbre</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagramme de Pagination d'Arbre" />
</div>

### Fetching Comments in The Context of a User

L’API `/comments` peut être utilisée dans deux contextes, pour différents cas d’utilisation :

- Pour renvoyer des commentaires triés et étiquetés avec des informations pour construire votre propre client.
  - Dans ce cas, définissez un paramètre de requête `contextUserId`.
- Pour récupérer des commentaires depuis votre backend pour des intégrations personnalisées.
  - La plateforme utilisera cela par défaut sans `contextUserId`. 

[inline-code-attrs-start title = 'Commentaires Pagination Précalculée'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires Pagination Flexible'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires Pagination Flexible dans le Contexte Utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Commentaires Pagination Flexible dans le Contexte Utilisateur pour les Commentaires de Niveau Supérieur Uniquement'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Il est possible d’obtenir les commentaires retournés sous forme d’arbre, la pagination ne comptant que les commentaires de niveau supérieur.

[inline-code-attrs-start title = 'Commentaires Sous forme d’Arbre dans le Contexte Utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vous voulez uniquement récupérer les commentaires de niveau supérieur et leurs enfants immédiats ? Voici une façon :

[inline-code-attrs-start title = 'Commentaires Sous forme d’Arbre avec Profondeur Maximale'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Cependant, dans votre interface utilisateur vous pourriez avoir besoin de savoir s’il faut afficher un bouton « afficher les réponses » sur chaque commentaire. Lors de la récupération des commentaires via un arbre, une propriété `hasChildren` est ajoutée aux commentaires lorsqu’elle est applicable.

### Get Comments as a Tree, Searching by Hash Tag

Il est possible de rechercher par hashtag en utilisant l’API, sur l’ensemble de votre locataire (pas limité à une page ou à `urlId`).

Dans cet exemple, nous omettons `urlId` et recherchons par plusieurs hashtags. L’API ne renverra que les commentaires contenant tous les hashtags demandés.

[inline-code-attrs-start title = 'Commentaires Sous forme d’Arbre dans le Contexte Utilisateur, par Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Structure de la Requête de Commentaires'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** L'urlId (URL de la page ou ID de l'article) auquel les commentaires sont associés. **/
    urlId?: string
    /** Limite les commentaires retournés par cet utilisateur. **/
    userId?: string
    /** Utilisez ceci pour rechercher par hashtag. Pour approfondir l’intersection de plusieurs hashtags, utilisez &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** La direction de tri. La valeur par défaut est MR (Most Relevant). Les autres options sont OF (Oldest First) et NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Pagination Précalculée : La page à récupérer, en commençant à 0. Passez -1 pour tous les commentaires (jusqu'à 250). **/
    page?: number
    /** Pagination Flexible : Combien de commentaires devons-nous renvoyer ? **/
    limit?: number
    /** Pagination Flexible : Combien de commentaires enfants devons-nous renvoyer pour chaque parent ? **/
    limitChildren?: number
    /** Pagination Flexible : Combien de commentaires devons-nous ignorer ? **/
    skip?: number
    /** Pagination Flexible : Combien de commentaires enfants devons-nous ignorer pour chaque parent ? **/
    skipChildren?: number
    /** Pour déterminer les commentaires bloqués et signalés. **/
    contextUserId?: string
    /** Pour déterminer les commentaires bloqués et signalés. **/
    anonUserId?: string
    /** Pour récupérer les commentaires enfants. **/
    parentId?: string
    /** Pour récupérer sous forme d’arbre. **/
    asTree?: boolean
    /** Jusqu’à quel niveau de l'arbre devons-nous retourner les données ? 0 ne retourne aucun enfant. 1 retourne les enfants immédiats, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Structure de la Réponse de Commentaires'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Helpful Tips

#### URL ID

Vous voudrez probablement utiliser l’API `Comment` avec le paramètre `urlId`. Vous pouvez d’abord appeler l’API `Pages` pour voir à quoi ressemblent les valeurs `urlId` disponibles.

#### Anonymous Actions

Pour les commentaires anonymes, vous voudrez probablement passer `anonUserId` lors de la récupération des commentaires, ainsi que lors du signalement et du blocage.

(!) Ceci est requis pour de nombreux magasins d’applications, car les utilisateurs doivent pouvoir signaler le contenu créé par les utilisateurs qu’ils voient, même s’ils ne sont pas connectés. Ne pas le faire peut entraîner la suppression de votre application de ce magasin.

#### Comments Not Being Returned

Vérifiez que vos commentaires sont approuvés et ne sont pas du spam.

---