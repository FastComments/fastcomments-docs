Page Reacts permet à vos utilisateurs d’aimer une page ou d’y réagir avec votre propre ensemble d’images de réaction. Le [widget Page Reacts](/guide-page-reacts.html) et le widget Floating Likes sont construits sur ces points de terminaison, et vous pouvez les appeler vous‑même pour créer votre propre bouton J’aime.

Contrairement au reste de ce guide, les points de terminaison Page Reacts sont publics. Ils sont appelés depuis le navigateur de vos utilisateurs, ne nécessitent aucune clé d’API et ne coûtent aucun crédit d’API. Chaque réaction appartient à l’utilisateur qui effectue la requête, de sorte qu’un utilisateur ne peut ajouter ou supprimer que ses propres réactions.

Il existe deux ensembles de points de terminaison :

- `/page-reacts/v1/likes/:tenantId` – un seul « like » par utilisateur et par page. Utilisez‑les pour un bouton J’aime.
- `/page-reacts/v2/:tenantId` – plusieurs réactions par page, chacune identifiée par un court `id` que vous choisissez (par exemple `heart` ou `laugh`).

Les deux sont également disponibles dans nos SDKs dans le cadre de l’`PublicApi`, par exemple `getV1PageLikes`, `createV1PageReact` et `deleteV1PageReact` dans le [SDK JavaScript](/guide-sdk-javascript.html).

### Identifying the User

Les réactions sont liées à l’utilisateur qui effectue la requête :

- **Utilisateurs SSO :** transmettez le paramètre de requête `sso`, défini sur le JSON encodé en URI du même objet SSO que vous fournissez au widget de commentaire. Voir [SSO](/guide-customizations-and-configuration.html#sso).
- **Utilisateurs anonymes :** lorsqu’il n’y a pas de paramètre `sso` et aucune connexion FastComments, le serveur attribue au navigateur un identifiant anonyme stocké dans le cookie de session FastComments. Envoyez les requêtes avec `credentials: 'include'` afin que le cookie soit conservé entre les requêtes. Les navigateurs qui bloquent les cookies tiers ne conserveront pas l’identifiant anonyme, utilisez donc SSO lorsque chaque utilisateur doit être reconnu de manière fiable.

### The urlId

`urlId` identifie la page, de la même façon que pour les commentaires. Utilisez le même `urlId` que vous fournissez au widget de commentaire afin que les likes et les commentaires soient comptés sur la même page. N’oubliez pas de l’encoder en URI.

[inline-code-attrs-start title = 'Exemple de bouton J\'aime'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Facultatif, pour les utilisateurs SSO. Le même objet que vous fournissez à l'option "sso" du widget de commentaire.
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