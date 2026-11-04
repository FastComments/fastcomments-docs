[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Esta API se utiliza para obtener comentarios para mostrarlos a un usuario. Por ejemplo, filtra automáticamente
los comentarios no aprobados o marcados como spam.

### Paginación

La paginación se puede hacer de dos maneras, dependiendo de los requisitos de rendimiento y del caso de uso:

1. La más rápida: **Paginación Precalculada**:
   1. Así funciona FastComments cuando utilizas nuestros widgets y clientes prediseñados.
   2. Hacer clic en "siguiente" simplemente incrementa el número de página.
   3. Puedes pensar en esto como si se recuperara de un almacén clave-valor.
   4. De esta manera, simplemente define un parámetro `page` que comience en `0` y una dirección de ordenación como `direction`.
   5. Los tamaños de página se pueden personalizar mediante reglas de personalización.
2. La más flexible: **Paginación Flexible**:
   1. De esta manera puedes definir parámetros personalizados `limit` y `skip`. No pases `page`.
   2. La `direction` de ordenación también es compatible.
   3. `limit` es el número total a devolver después de aplicar `skip`.
      - Ejemplo: establece `skip = 200, limit = 100` cuando `page size = 100` y `page = 2`.
   4. Los comentarios hijos todavía cuentan en la paginación. Puedes evitar esto usando la opción `asTree`.
      - Puedes paginar los hijos mediante `limitChildren` y `skipChildren`.
      - Puedes limitar la profundidad de los hilos devueltos mediante `maxTreeDepth`.

### Hilos

1. Al usar `Precalculated Pagination`, los comentarios se agrupan por *página* y los comentarios en hilos afectan la página general.
   1. De esta manera, los hilos se pueden determinar en el cliente basándose en `parentId`.
   2. Por ejemplo, con una página que tiene un comentario de nivel superior y 29 respuestas, y estableciendo `page=0` en la API - obtendrás solo el comentario de nivel superior y los 29 hijos.
2. Al usar `Flexible Pagination`, puedes definir un parámetro `parentId`.
   1. Establece esto en null para obtener solo los comentarios de nivel superior.
   2. Luego, para ver los hilos, llama a la API de nuevo y pasa `parentId`.
   3. Una solución común es hacer una llamada a la API para los comentarios de nivel superior y luego hacer llamadas paralelas a la API para obtener los comentarios de los hijos de cada comentario.
3. __NUEVO A partir de Feb 2023!__ Obtener como un árbol usando `&asTree=true`.
   1. Puedes pensar en esto como `Flexible Pagination as a Tree`.
   2. Solo los comentarios de nivel superior cuentan en la paginación.
   3. Establece `parentId=null` para comenzar el árbol en la raíz (debes establecer `parentId`).
   4. Establece `skip` y `limit` para la paginación.
   5. Establece `asTree` en `true`.
   6. El costo en créditos aumenta `2x`, ya que nuestro backend tiene que hacer mucho más trabajo en este escenario.
   7. Establece `maxTreeDepth`, `limitChildren` y `skipChildren` según lo desees.

### Árboles Explicados

Al usar `asTree`, puede ser difícil razonar sobre la paginación. Aquí hay una gráfica útil:

<div class="screenshot white-bg">
    <div class="title">Diagrama de Paginación en Árbol</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagrama de Paginación en Árbol" />
</div>

### Obtención de Comentarios en el Contexto de un Usuario

La API `/comments` se puede usar en dos contextos, para diferentes casos de uso:

- Para devolver comentarios ordenados y etiquetados con información para construir tu propio cliente.
  - En este caso, define un parámetro de consulta `contextUserId`.
- Para obtener comentarios desde tu backend para integraciones personalizadas.
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

### Obtener Comentarios como un Árbol

Es posible obtener los comentarios devueltos como un árbol, con la paginación contando solo los comentarios de nivel superior.

[inline-code-attrs-start title = 'Comentarios Como Árbol en Contexto de Usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

¿Quieres obtener solo los comentarios de nivel superior y los hijos inmediatos? Aquí hay una manera:

[inline-code-attrs-start title = 'Comentarios Como Árbol con Profundidad Máxima'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Sin embargo, en tu interfaz de usuario podrías necesitar saber si mostrar un botón de "mostrar respuestas" en
cada comentario. Al obtener comentarios mediante un árbol, hay una propiedad `hasChildren` etiquetada
en los comentarios cuando aplica.

### Obtener Comentarios como un Árbol, Buscando por Hashtag

Es posible buscar por hashtag usando la API, a través de todo tu tenant (no limitado a una página, o `urlId`).

En este ejemplo, omitimos `urlId`, y buscamos por múltiples hashtags. La API solo devolverá comentarios que tengan todos los hashtags solicitados.

[inline-code-attrs-start title = 'Comentarios Como Árbol en Contexto de Usuario, Por Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Todos los Parámetros de la Solicitud

[inline-code-attrs-start title = 'Estructura de la Solicitud de Comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** El urlId (url de la página, o id del artículo) al que están asociados los comentarios. **/
    urlId?: string
    /** Limita los comentarios devueltos a este usuario. **/
    userId?: string
    /** Usa esto para buscar por hashtag. Para profundizar en la intersección de múltiples hashtags, haz &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** La dirección de ordenación. Por defecto es MR (Más Relevante). Otras opciones son OF (Más Antiguos Primero) y NF (Más Nuevos Primero). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Paginación Precalculada: La página a obtener, comenzando con 0. Pasa -1 para todos los comentarios (hasta 250). **/
    page?: number
    /** Paginación Flexible: ¿Cuántos comentarios debemos devolver? **/
    limit?: number
    /** Paginación Flexible: ¿Cuántos comentarios hijos debemos devolver por cada padre? **/
    limitChildren?: number
    /** Paginación Flexible: ¿Cuántos comentarios debemos omitir? **/
    skip?: number
    /** Paginación Flexible: ¿Cuántos comentarios hijos debemos omitir por cada padre? **/
    skipChildren?: number
    /** Para determinar comentarios bloqueados y marcados. **/
    contextUserId?: string
    /** Para determinar comentarios bloqueados y marcados. **/
    anonUserId?: string
    /** Para obtener comentarios hijos. **/
    parentId?: string
    /** Para obtener como un árbol. **/
    asTree?: boolean
    /** ¿Qué tan profundo en el árbol debemos devolver datos? 0 no devuelve hijos. 1 devuelve los hijos inmediatos, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### La Respuesta

[inline-code-attrs-start title = 'Estructura de la Respuesta de Comentarios'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de fallo. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Incluido en caso de fallo. **/
    reason?: string
    /** ¡Los comentarios! **/
    comments: Comment[]
}
[inline-code-end]

### Consejos Útiles

#### URL ID

Probablemente quieras usar la API `Comment` con el parámetro `urlId`. Puedes llamar primero a la API `Pages`, para ver cómo son los valores de `urlId` disponibles para ti. 

#### Acciones Anónimas

Para comentarios anónimos probablemente quieras pasar `anonUserId` al obtener comentarios, y al realizar marcado y bloqueo.

(!) Esto es requerido por muchas tiendas de aplicaciones, ya que los usuarios deben poder marcar contenido creado por usuarios que puedan ver, incluso si no han iniciado sesión. No hacerlo puede causar que tu aplicación sea eliminada de dicha tienda.

#### Comentarios Que No Se Devuelven

Verifica que tus comentarios estén aprobados y que no sean spam.