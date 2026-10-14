Page Reacts permite a tus usuarios dar me gusta a una página, o reaccionar a ella con tu propio conjunto de imágenes de reacción. El widget [Page Reacts](/guide-page-reacts.html) y el widget Floating Likes están construidos sobre estos endpoints, y puedes llamarlos tú mismo para crear tu propio botón de me gusta.

A diferencia del resto de esta guía, los endpoints de Page Reacts son públicos. Se llaman desde los navegadores de tus usuarios, no requieren una clave API y no consumen créditos de API. Cada reacción pertenece al usuario que realiza la solicitud, por lo que un usuario solo puede agregar o eliminar sus propias reacciones.

Hay dos conjuntos de endpoints:

- `/page-reacts/v1/likes/:tenantId` - un solo "like" por usuario por página. Usa estos para un botón de me gusta.
- `/page-reacts/v2/:tenantId` - múltiples reacciones por página, cada una identificada por un `id` corto que elijas (por ejemplo `heart` o `laugh`).

Ambos también están disponibles en nuestros SDKs como parte de la `PublicApi`, por ejemplo `getV1PageLikes`, `createV1PageReact` y `deleteV1PageReact` en el [JavaScript SDK](/guide-sdk-javascript.html).

### Mostrar quién reaccionó

Pasa `includeUserInfo=public` a los endpoints `GET` para también obtener los usuarios que dieron me gusta o reaccionaron, con sus nombres de usuario y avatares, los más recientes primero. Los usuarios que han configurado su actividad de perfil como privada se omiten, al igual que los usuarios anónimos. Se devuelven hasta 100 reacciones.

Los usuarios SSO se crean con actividad de perfil privada a menos que tu carga útil SSO establezca `isProfileActivityPrivate` a `false`, así que configúralo para los usuarios que deseas que aparezcan en estas listas.

En páginas que requieren SSO, solo los usuarios SSO que pueden ver la página pueden listar quién reaccionó. Los demás reciben un `403` con el código `sso-required` o `page-forbidden`.

Desde tu servidor, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) y [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) devuelven las mismas listas con una clave API, y también aceptan `includeUserInfo=include-private` para incluir a los usuarios con perfiles privados.

### Identificar al usuario

Las reacciones están vinculadas al usuario que realiza la solicitud:

- **Usuarios SSO:** pasa el parámetro de consulta `sso`, configurado con el JSON codificado en URI del mismo objeto SSO que le das al widget de comentarios. Ver [SSO](/guide-customizations-and-configuration.html#sso).
- **Usuarios anónimos:** cuando no hay parámetro `sso` y no hay inicio de sesión en FastComments, el servidor asigna al navegador un id anónimo almacenado en la cookie de sesión de FastComments. Envía solicitudes con `credentials: 'include'` para que la cookie se mantenga entre peticiones. Los navegadores que bloquean cookies de terceros no conservarán el id anónimo, así que usa SSO cuando cada usuario deba ser reconocido de forma fiable.

### El urlId

`urlId` identifica la página, igual que lo hace para los comentarios. Usa el mismo `urlId` que le das al widget de comentarios para que los me gusta y los comentarios se cuenten en la misma página. Recuerda codificarlo en URI.

[inline-code-attrs-start title = 'Ejemplo de Botón de Me Gusta'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]

---