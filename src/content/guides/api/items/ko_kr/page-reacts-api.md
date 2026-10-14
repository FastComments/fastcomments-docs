Page Reacts는 사용자가 페이지에 좋아요를 누르거나 자체 반응 이미지 세트로 반응할 수 있게 합니다. [Page Reacts widget](/guide-page-reacts.html)와 Floating Likes 위젯은 이러한 엔드포인트를 기반으로 구축되며, 직접 호출하여 자체 좋아요 버튼을 만들 수 있습니다.

이 가이드의 다른 부분과 달리, Page Reacts 엔드포인트는 공개되어 있습니다. 사용자의 브라우저에서 호출되며 API 키가 필요 없고 API 크레딧도 소모되지 않습니다. 모든 반응은 요청을 보낸 사용자에게 귀속되므로 사용자는 자신의 반응만 추가하거나 제거할 수 있습니다.

엔드포인트는 두 가지 세트가 있습니다:

- `/page-reacts/v1/likes/:tenantId` - 사용자당 페이지당 하나의 "like"만 허용합니다. 좋아요 버튼에 사용하세요.
- `/page-reacts/v2/:tenantId` - 페이지당 여러 반응을 허용하며, 각각은 사용자가 선택한 짧은 `id`(예: `heart` 또는 `laugh`)로 식별됩니다.

두 엔드포인트는 `PublicApi`의 일부로서 우리 SDK에서도 사용할 수 있으며, 예를 들어 [JavaScript SDK](/guide-sdk-javascript.html)에서는 `getV1PageLikes`, `createV1PageReact`, `deleteV1PageReact`와 같은 메서드가 제공됩니다.

### 반응한 사람 보기

`GET` 엔드포인트에 `includeUserInfo=public`을 전달하면 좋아요 또는 반응을 한 사용자들의 사용자명과 아바타를 최신 순으로 함께 가져올 수 있습니다. 프로필 활동을 비공개로 설정한 사용자와 익명 사용자는 제외됩니다. 최대 100개의 반응이 반환됩니다.

SSO 사용자는 `isProfileActivityPrivate`를 `false`로 설정하지 않는 한 프로필 활동이 비공개 상태로 생성됩니다. 따라서 목록에 표시하고 싶은 사용자는 해당 값을 설정해 주세요.

SSO가 필요한 페이지에서는 해당 페이지를 볼 수 있는 SSO 사용자만 반응한 사람 목록을 조회할 수 있습니다. 그 외의 경우 `sso-required` 또는 `page-forbidden` 코드를 포함한 `403` 오류가 반환됩니다.

서버에서 [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get)와 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get)를 호출하면 API 키와 함께 동일한 목록을 반환하며, `includeUserInfo=include-private`를 사용하면 비공개 프로필을 가진 사용자도 포함됩니다.

### 사용자를 식별하기

반응은 요청을 보낸 사용자와 연결됩니다:

- **SSO 사용자:** `sso` 쿼리 매개변수를 전달하고, 댓글 위젯에 제공한 것과 동일한 SSO 객체를 URI 인코딩한 JSON으로 설정합니다. 자세한 내용은 [SSO](/guide-customizations-and-configuration.html#sso)를 참고하세요.
- **익명 사용자:** `sso` 매개변수가 없고 FastComments 로그인도 없는 경우, 서버는 FastComments 세션 쿠키에 저장된 익명 ID를 브라우저에 할당합니다. 요청을 보낼 때 `credentials: 'include'`를 사용하여 쿠키가 요청 간에 유지되도록 합니다. 서드파티 쿠키를 차단하는 브라우저는 익명 ID를 유지하지 않으므로, 각 사용자를 신뢰성 있게 인식해야 할 경우 SSO를 사용하세요.

### urlId

`urlId`는 페이지를 식별하며, 댓글에서도 동일하게 사용됩니다. 좋아요와 댓글이 같은 페이지에서 집계되도록 댓글 위젯에 제공한 것과 동일한 `urlId`를 사용하세요. URI 인코딩을 잊지 마세요.

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