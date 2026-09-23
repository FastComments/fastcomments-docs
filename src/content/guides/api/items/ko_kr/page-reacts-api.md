Page Reacts는 사용자가 페이지에 좋아요를 누르거나 자체 반응 이미지 세트로 반응할 수 있게 합니다. The [Page Reacts widget](/guide-page-reacts.html)와 Floating Likes 위젯은 이 엔드포인트를 기반으로 구축되며, 직접 호출하여 자체 좋아요 버튼을 만들 수 있습니다.

이 가이드의 다른 부분과 달리, Page Reacts 엔드포인트는 공개되어 있습니다. 사용자의 브라우저에서 호출되며 API 키가 필요 없고 API 크레딧도 소모되지 않습니다. 각 반응은 요청을 보낸 사용자에게 귀속되므로 사용자는 자신의 반응만 추가하거나 제거할 수 있습니다.

엔드포인트는 두 가지 세트가 있습니다:

- `/page-reacts/v1/likes/:tenantId` - 사용자당 페이지당 하나의 "like"를 제공합니다. 좋아요 버튼에 사용하세요.
- `/page-reacts/v2/:tenantId` - 페이지당 여러 반응을 지원하며, 각각은 사용자가 선택한 짧은 `id`(예: `heart` 또는 `laugh`)로 식별됩니다.

두 엔드포인트 모두 `PublicApi`의 일부로 SDK에서도 사용할 수 있으며, 예를 들어 [JavaScript SDK](/guide-sdk-javascript.html)에서 `getV1PageLikes`, `createV1PageReact`, `deleteV1PageReact` 등이 있습니다.

### Identifying the User

반응은 요청을 보낸 사용자와 연결됩니다:

- **SSO 사용자:** `sso` 쿼리 매개변수를 전달하고, 댓글 위젯에 제공한 동일한 SSO 객체를 URI 인코딩한 JSON으로 설정합니다. See [SSO](/guide-customizations-and-configuration.html#sso).
- **익명 사용자:** `sso` 매개변수가 없고 FastComments 로그인이 없을 경우, 서버는 FastComments 세션 쿠키에 저장된 익명 ID를 브라우저에 할당합니다. 요청을 보낼 때 `credentials: 'include'`를 사용하여 쿠키가 요청 간에 유지되도록 합니다. 제3자 쿠키를 차단하는 브라우저는 익명 ID를 유지하지 않으므로, 각 사용자를 신뢰성 있게 인식해야 할 경우 SSO를 사용하세요.

### The urlId

`urlId`는 페이지를 식별하며, 댓글에서도 동일하게 사용됩니다. 좋아요와 댓글이 같은 페이지에서 집계되도록 댓글 위젯에 제공한 동일한 `urlId`를 사용하세요. URI 인코딩을 잊지 마세요.

[inline-code-attrs-start title = '좋아요 버튼 예시'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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