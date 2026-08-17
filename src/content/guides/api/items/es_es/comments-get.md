[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Esta API se utiliza para obtener comentarios para mostrarlos a un usuario. Por ejemplo, filtra automáticamente los comentarios no aprobados o spam.

### Pagination

La paginación puede realizarse de una de dos maneras, según los requisitos de rendimiento y el caso de uso:

1. **Más rápido: Paginación precalculada**:
   1. Así es como FastComments funciona cuando utilizas nuestros widgets y clientes preconstruidos.
   2. Hacer clic en "next" simplemente incrementa el número de página.
   3. Puedes pensar en esto como una recuperación desde una tienda clave‑valor.
   4. De esta forma, simplemente define un parámetro `page` que comience en `0` y una dirección de ordenamiento como `direction`.
   5. Los tamaños de página pueden personalizarse mediante reglas de personalización.
2. **Más flexible: Paginación flexible**:
   1. De esta manera puedes definir parámetros personalizados `limit` y `skip`. No pases `page`.
   2. También se admite el `direction` de ordenamiento.
   3. `limit` es el número total a devolver después de aplicar `skip`.
      - Ejemplo: establece `skip = 200, limit = 100` cuando `page size = 100` y `page = 2`.
   4. Los comentarios hijos siguen contando en la paginación. Puedes evitarlo usando la opción `asTree`.
      - Puedes paginar los hijos mediante `limitChildren` y `skipChildren`.
      - Puedes limitar la profundidad de los hilos devueltos mediante `maxTreeDepth`.

### Threads

1. Cuando se usa `Precalculated Pagination`, los comentarios se agrupan por *page* y los comentarios en hilos afectan la página global.
   1. De esta forma, los hilos pueden determinarse en el cliente basándose en `parentId`.
   2. Por ejemplo, con una página que tiene un comentario de nivel superior y 29 respuestas, y estableciendo `page=0` en la API, obtendrás solo el comentario de nivel superior y los 29 hijos.
2. Cuando se usa `Flexible Pagination`, puedes definir un parámetro `parentId`.
   1. Establécelo en null para obtener solo los comentarios de nivel superior.
   2. Luego, para ver los hilos, llama a la API nuevamente y pasa `parentId`.
   3. Una solución común es hacer una llamada a la API para los comentarios de nivel superior y luego llamadas paralelas para obtener los comentarios de los hijos de cada comentario.
3. __NEW As of Feb 2023!__ Obtén los datos como un árbol usando `&asTree=true`.
   1. Puedes pensar en esto como `Flexible Pagination as a Tree`.
   2. Solo los comentarios de nivel superior cuentan en la paginación.
   3. Establece `parentId=null` para iniciar el árbol en la raíz (debes establecer `parentId`).
   4. Establece `skip` y `limit` para la paginación.
   5. Establece `asTree` en `true`.
   6. El costo de créditos aumenta a `2x`, ya que nuestro backend debe hacer mucho más trabajo en este escenario.
   7. Establece `maxTreeDepth`, `limitChildren` y `skipChildren` según lo desees.

### Trees Explained

Cuando se usa `asTree`, puede ser difícil razonar sobre la paginación. Aquí tienes un gráfico útil:

<div class="screenshot white-bg">
    <div class="title">Diagrama de paginación de árbol</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagrama de paginación de árbol" />
</div>

### Fetching Comments in The Context of a User

La API `/comments` puede usarse en dos contextos, para diferentes casos de uso:

- Para devolver comentarios ordenados y etiquetados con información para construir tu propio cliente.
  - En este caso, define un parámetro de consulta `contextUserId`.
- Para obtener comentarios de tu backend para integraciones personalizadas.
  - La plataforma usará esto por defecto sin `contextUserId`. 

[inline-code-attrs-start title = 'Comentarios Paginación Precalculada'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Comentarios Paginación Flexible'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Comentarios Paginación Flexible en Contexto de Usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Comentarios Paginación Flexible en Contexto de Usuario solo Comentarios de Nivel Superior'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Es posible obtener los comentarios devueltos como un árbol, con la paginación contando solo los comentarios de nivel superior.

[inline-code-attrs-start title = 'Comentarios como árbol en contexto de usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

¿Quieres obtener solo los comentarios de nivel superior y sus hijos inmediatos? Aquí tienes una forma:

[inline-code-attrs-start title = 'Comentarios como árbol con profundidad máxima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Sin embargo, en tu UI podrías necesitar saber si mostrar un botón "mostrar respuestas" en cada comentario. Al obtener comentarios mediante un árbol, existe una propiedad `hasChildren` etiquetada en los comentarios cuando corresponde.

### Get Comments as a Tree, Searching by Hash Tag

Es posible buscar por hashtag usando la API, en todo tu tenant (no limitado a una página o `urlId`).

En este ejemplo, omitimos `urlId` y buscamos por varios hashtags. La API solo devolverá los comentarios que tengan todos los hashtags solicitados.

[inline-code-attrs-start title = 'Comentarios como árbol en contexto de usuario, por etiqueta hash'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Estructura de solicitud de comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** El urlId (URL de la página o ID del artículo) con el que están asociados los comentarios. **/
    urlId?: string
    /** Limitar los comentarios devueltos por este usuario. **/
    userId?: string
    /** Use esto para buscar por etiqueta hash. Para profundizar en la intersección de múltiples etiquetas hash, use &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** La dirección de ordenamiento. Por defecto es MR (Más relevante). Otras opciones son OF (Más antiguo primero) y NF (Más nuevo primero). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Paginación precalculada: La página a obtener, comenzando en 0. Pase -1 para todos los comentarios (hasta 250). **/
    page?: number
    /** Paginación flexible: ¿Cuántos comentarios debemos devolver? **/
    limit?: number
    /** Paginación flexible: ¿Cuántos comentarios hijos debemos devolver por cada padre? **/
    limitChildren?: number
    /** Paginación flexible: ¿Cuántos comentarios debemos omitir? **/
    skip?: number
    /** Paginación flexible: ¿Cuántos comentarios hijos debemos omitir por cada padre? **/
    skipChildren?: number
    /** Para determinar comentarios bloqueados y marcados. **/
    contextUserId?: string
    /** Para determinar comentarios bloqueados y marcados. **/
    anonUserId?: string
    /** Para obtener comentarios hijos. **/
    parentId?: string
    /** Para obtener como árbol. **/
    asTree?: boolean
    /** ¿Qué tan profundo en el árbol debemos devolver datos? 0 no devuelve hijos. 1 devuelve hijos inmediatos, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Estructura de respuesta de comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Incluido en caso de error. **/
    reason?: string
    /** ¡Los comentarios! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Probablemente quieras usar la API `Comment` con el parámetro `urlId`. Puedes llamar primero a la API `Pages` para ver cómo se ven los valores `urlId` disponibles para ti.

#### Anonymous Actions

Para comentar de forma anónima probablemente quieras pasar `anonUserId` al obtener comentarios y al realizar marcados y bloqueos.

(!) Esto es requerido por muchas tiendas de aplicaciones, ya que los usuarios deben poder marcar contenido creado por usuarios que pueden ver, incluso si no han iniciado sesión. No hacerlo puede causar que tu aplicación sea eliminada de dicha tienda.

#### Comments Not Being Returned

Verifica que tus comentarios estén aprobados y que no sean spam.

---