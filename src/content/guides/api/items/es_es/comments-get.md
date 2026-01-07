[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Esta API se usa para obtener comentarios para mostrar a un usuario. Por ejemplo, filtra automáticamente
los comentarios no aprobados o spam.

### Paginación

La paginación se puede hacer de dos maneras, dependiendo de los requisitos de rendimiento y caso de uso:

1. Más rápido: **Paginación Precalculada**:
   1. Así es como funciona FastComments cuando usa nuestros widgets y clientes preconstruidos.
   2. Hacer clic en "siguiente" simplemente aumenta el conteo de páginas.
   3. Puede pensar en esto como ser recuperado por un almacén de clave-valor.
   4. De esta manera, simplemente defina un parámetro `page` comenzando en `0` y una dirección de ordenamiento como `direction`.
   5. Los tamaños de página se pueden personalizar a través de reglas de personalización.
2. Más flexible: **Paginación Flexible**:
   1. De esta manera puede definir parámetros personalizados `limit` y `skip`. No pase `page`.
   2. También se soporta la `direction` de ordenamiento.
   3. `limit` es el total a devolver después de que se aplica `skip`.
      - Ejemplo: establezca `skip = 200, limit = 100` cuando `tamaño de página = 100` y `page = 2`.
   4. Los comentarios hijos aún cuentan en la paginación. Puede evitar esto usando la opción `asTree`.
      - Puede paginar hijos a través de `limitChildren` y `skipChildren`.
      - Puede limitar la profundidad de los hilos devueltos a través de `maxTreeDepth`.

### Hilos

1. Cuando usa `Paginación Precalculada`, los comentarios se agrupan por *página* y los comentarios en hilos afectan la página general.
   1. De esta manera, los hilos se pueden determinar en el cliente basándose en `parentId`.
   2. Por ejemplo, con una página con un comentario de nivel superior, y 29 respuestas, y configurando `page=0` en la API - obtendrá solo el comentario de nivel superior y los 29 hijos.
   3. [Imagen de ejemplo aquí ilustrando múltiples páginas.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Cuando usa `Paginación Flexible`, puede definir un parámetro `parentId`.
   1. Establezca esto a null para obtener solo comentarios de nivel superior.
   2. Luego para ver hilos, llame a la API nuevamente y pase `parentId`.
   3. Una solución común es hacer una llamada a la API para los comentarios de nivel superior y luego hacer llamadas paralelas a la API para obtener comentarios para los hijos de cada comentario.
3. __¡NUEVO Desde Feb 2023!__ Obtener como árbol usando `&asTree=true`.
   1. Puede pensar en esto como `Paginación Flexible como Árbol`.
   2. Solo los comentarios de nivel superior cuentan en la paginación.
   3. Establezca `parentId=null` para iniciar el árbol en la raíz (debe establecer `parentId`).
   4. Establezca `skip` y `limit` para paginación.
   5. Establezca `asTree` a `true`.
   6. El costo de créditos aumenta por `2x`, ya que nuestro backend tiene que hacer mucho más trabajo en este escenario.
   7. Establezca `maxTreeDepth`, `limitChildren` y `skipChildren` según lo deseado.

### Árboles Explicados

Cuando usa `asTree`, puede ser difícil razonar sobre la paginación. Aquí hay un gráfico útil:

<div class="screenshot white-bg">
    <div class="title">Diagrama de Paginación de Árbol</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagrama de Paginación de Árbol" />
</div>

### Obtener Comentarios en el Contexto de un Usuario

La API `/comments` se puede usar en dos contextos, para diferentes casos de uso:

- Para devolver comentarios ordenados y etiquetados con información para construir su propio cliente.
  - En este caso, defina un parámetro de consulta `contextUserId`.
- Para obtener comentarios desde su backend para integraciones personalizadas.
  - La plataforma usará esto por defecto sin `contextUserId`.

[inline-code-attrs-start title = 'Paginación Precalculada de Comentarios'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Paginación Flexible de Comentarios'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Paginación Flexible de Comentarios en Contexto de Usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Paginación Flexible de Comentarios en Contexto de Usuario Solo para Comentarios de Nivel Superior'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Obtener Comentarios como Árbol

Es posible obtener los comentarios devueltos como un árbol, con la paginación contando solo los comentarios de nivel superior.

[inline-code-attrs-start title = 'Comentarios Como Árbol en Contexto de Usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

¿Quiere obtener solo los comentarios de nivel superior y los hijos inmediatos? Aquí hay una forma:

[inline-code-attrs-start title = 'Comentarios Como Árbol con Profundidad Máxima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Sin embargo, en su UI podría necesitar saber si mostrar un botón de "mostrar respuestas" en
cada comentario. Cuando obtiene comentarios a través de un árbol hay una propiedad `hasChildren` etiquetada
en los comentarios cuando aplica.

### Obtener Comentarios como Árbol, Buscando por Hash Tag

Es posible buscar por hashtag usando la API, a través de todo su inquilino (no limitado a una página, o `urlId`).

En este ejemplo, omitimos `urlId`, y buscamos por múltiples hashtags. La API solo devolverá comentarios que tengan todos los hashtags solicitados.

[inline-code-attrs-start title = 'Comentarios Como Árbol en Contexto de Usuario, Por Hash Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Todos los Parámetros de Solicitud

[inline-code-attrs-start title = 'Estructura de Solicitud de Comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### La Respuesta

[inline-code-attrs-start title = 'Estructura de Respuesta de Comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Consejos Útiles

#### URL ID

Probablemente quiera usar la API `Comment` con el parámetro `urlId`. Puede llamar a la API `Pages` primero, para ver cómo lucen los valores `urlId` disponibles para usted.

#### Acciones Anónimas

Para comentarios anónimos probablemente quiera pasar `anonUserId` cuando obtiene comentarios, y cuando realiza marcado y bloqueo.

(!) Esto es requerido para muchas tiendas de aplicaciones ya que los usuarios deben poder marcar contenido creado por usuarios que pueden ver, incluso si no han iniciado sesión. No hacerlo puede causar que su aplicación sea eliminada de dicha tienda.

#### Comentarios No Devueltos

Verifique que sus comentarios estén aprobados, y no sean spam.
