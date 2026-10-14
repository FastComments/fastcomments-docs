Page Reacts permite que seus usuários curtam uma página, ou reajam a ela com seu próprio conjunto de imagens de reação. O widget [Page Reacts](/guide-page-reacts.html) e o widget Floating Likes são construídos sobre esses endpoints, e você pode chamá-los você mesmo para criar seu próprio botão de curtir.

Ao contrário do resto deste guia, os endpoints do Page Reacts são públicos. Eles são chamados a partir dos navegadores de seus usuários, não exigem chave de API e não custam créditos de API. Cada reação pertence ao usuário que faz a solicitação, portanto um usuário só pode adicionar ou remover a sua própria.

Existem dois conjuntos de endpoints:

- `/page-reacts/v1/likes/:tenantId` - um único "like" por usuário por página. Use estes para um botão de curtir.
- `/page-reacts/v2/:tenantId` - múltiplas reações por página, cada uma identificada por um curto `id` que você escolhe (por exemplo `heart` ou `laugh`).

Ambos também estão disponíveis em nossos SDKs como parte da `PublicApi`, por exemplo `getV1PageLikes`, `createV1PageReact` e `deleteV1PageReact` no [JavaScript SDK](/guide-sdk-javascript.html).

### Mostrando quem reagiu

Passe `includeUserInfo=public` para os endpoints `GET` para também obter os usuários que curtiram ou reagiram, com seus nomes de usuário e avatares, mais recentes primeiro. Usuários que tornaram sua atividade de perfil privada são excluídos, assim como usuários anônimos. Até 100 reações são retornadas.

Usuários SSO são criados com atividade de perfil privada, a menos que sua carga útil SSO defina `isProfileActivityPrivate` como `false`, portanto configure isso para os usuários que você deseja que apareçam nessas listas.

Em páginas que exigem SSO, somente usuários SSO que podem visualizar a página podem listar quem reagiu. Outros recebem um `403` com o código `sso-required` ou `page-forbidden`.

Do seu servidor, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) e [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) retornam as mesmas listas com uma chave de API, e também aceitam `includeUserInfo=include-private` para incluir usuários com perfis privados.

### Identificando o Usuário

Reactions são vinculadas ao usuário que faz a solicitação:

- **Usuários SSO:** passe o parâmetro de consulta `sso`, definido como o JSON codificado em URI do mesmo objeto SSO que você fornece ao widget de comentários. Veja [SSO](/guide-customizations-and-configuration.html#sso).
- **Usuários anônimos:** quando não há parâmetro `sso` e não há login no FastComments, o servidor atribui ao navegador um id anônimo armazenado no cookie de sessão do FastComments. Envie solicitações com `credentials: 'include'` para que o cookie seja mantido entre as requisições. Navegadores que bloqueiam cookies de terceiros não manterão o id anônimo, portanto use SSO quando cada usuário precisar ser reconhecido de forma confiável.

### O urlId

`urlId` identifica a página, da mesma forma que faz para comentários. Use o mesmo `urlId` que você fornece ao widget de comentários para que curtidas e comentários sejam contados na mesma página. Lembre-se de codificá-lo em URI.

[inline-code-attrs-start title = 'Exemplo de Botão de Curtida'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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