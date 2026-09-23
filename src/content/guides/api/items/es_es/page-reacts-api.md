---
Page Reacts permite a tus usuarios dar me gusta a una página, o reaccionar a ella con tu propio conjunto de imágenes de reacción. El [Page Reacts widget](/guide-page-reacts.html) y el widget Floating Likes se construyen sobre estos endpoints, y puedes llamarlos tú mismo para crear tu propio botón de me gusta.

A diferencia del resto de esta guía, los endpoints de Page Reacts son públicos. Se llaman desde los navegadores de tus usuarios, no requieren una clave API y no consumen créditos de API. Cada reacción pertenece al usuario que hace la solicitud, por lo que un usuario solo puede agregar o eliminar sus propias.

Hay dos conjuntos de endpoints:

- `/page-reacts/v1/likes/:tenantId` - un único "like" por usuario por página. Usa estos para un botón de me gusta.
- `/page-reacts/v2/:tenantId` - múltiples reacciones por página, cada una identificada por un `id` corto que elijas (por ejemplo `heart` o `laugh`).

Ambos también están disponibles en nuestros SDKs como parte de la `PublicApi`, por ejemplo `getV1PageLikes`, `createV1PageReact` y `deleteV1PageReact` en el [JavaScript SDK](/guide-sdk-javascript.html).

### Identificando al Usuario

Las reacciones están vinculadas al usuario que hace la solicitud:

- **Usuarios SSO:** pasa el parámetro de consulta `sso`, configurado con el JSON codificado en URI del mismo objeto SSO que le das al widget de comentarios. Ver [SSO](/guide-customizations-and-configuration.html#sso).
- **Usuarios anónimos:** cuando no hay parámetro `sso` y no hay inicio de sesión en FastComments, el servidor asigna al navegador un id anónimo almacenado en la cookie de sesión de FastComments. Envía solicitudes con `credentials: 'include'` para que la cookie se mantenga entre peticiones. Los navegadores que bloquean cookies de terceros no conservarán el id anónimo, por lo que debes usar SSO cuando cada usuario debe ser reconocido de forma fiable.

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