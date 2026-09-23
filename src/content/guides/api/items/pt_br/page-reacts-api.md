Page Reacts permite que seus usuários curtam uma página, ou reajam a ela com seu próprio conjunto de imagens de reação. O [Page Reacts widget](/guide-page-reacts.html) e o widget Floating Likes são construídos sobre esses endpoints, e você pode chamá‑los diretamente para criar seu próprio botão de curtir.

Ao contrário do restante deste guia, os endpoints do Page Reacts são públicos. Eles são chamados a partir dos navegadores de seus usuários, não exigem chave de API e não custam créditos de API. Cada reação pertence ao usuário que faz a solicitação, portanto um usuário só pode adicionar ou remover a própria.

Existem dois conjuntos de endpoints:

- `/page-reacts/v1/likes/:tenantId` – um único “curtir” por usuário por página. Use este para um botão de curtir.  
- `/page-reacts/v2/:tenantId` – múltiplas reações por página, cada uma identificada por um `id` curto que você escolhe (por exemplo `heart` ou `laugh`).

Ambos também estão disponíveis em nossos SDKs como parte da `PublicApi`, por exemplo `getV1PageLikes`, `createV1PageReact` e `deleteV1PageReact` no [JavaScript SDK](/guide-sdk-javascript.html).

### Identificando o Usuário

Reações são vinculadas ao usuário que faz a solicitação:

- **Usuários SSO:** passe o parâmetro de consulta `sso`, definido como o JSON codificado em URI do mesmo objeto SSO que você fornece ao widget de comentários. Veja [SSO](/guide-customizations-and-configuration.html#sso).  
- **Usuários anônimos:** quando não há parâmetro `sso` e nenhum login FastComments, o servidor atribui ao navegador um id anônimo armazenado no cookie de sessão FastComments. Envie solicitações com `credentials: 'include'` para que o cookie seja mantido entre as requisições. Navegadores que bloqueiam cookies de terceiros não manterão o id anônimo, portanto use SSO quando cada usuário precisar ser reconhecido de forma confiável.

### O urlId

`urlId` identifica a página, da mesma forma que faz para comentários. Use o mesmo `urlId` que você fornece ao widget de comentários para que curtidas e comentários sejam contados na mesma página. Lembre‑se de codificá‑lo em URI.

[inline-code-attrs-start title = 'Exemplo de Botão Curtir'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcional, para usuários SSO. O mesmo objeto que você fornece à opção "sso" do widget de comentários.
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