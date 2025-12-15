자주 접하는 몇 가지 증상과 일반적인 해결책입니다.

### "This is a demo" 메시지

이것은 데모 테넌트를 사용하는 저희 홈페이지에서 위젯 코드를 복사했을 때 표시됩니다. 귀하의 테넌트를 사용하려면 [여기](https://fastcomments.com/auth/my-account/get-acct-code)에서 위젯 코드를 복사하세요.

### "FastComments cannot load on this domain" 오류

FastComments는 귀하의 계정과 관련된 요청을 인증하기 위해 어떤 도메인이 귀하의 것인지 알아야 합니다. 이 오류를 해결하는 방법은 [저희 문서를 확인하세요](/guide-multiple-sites.html#add-domains-to-account)(계정에 정확한 서브도메인 + 도메인을 추가하기만 하면 됩니다).

이것은 평가판 기간이 끝난 후에만 발생해야 합니다. 평가판 기간 동안에는 새 도메인의 모든 요청이 자동으로 계정에 추가됩니다.

### 커스텀 설치에서 마이그레이션된 댓글이 표시되지 않음

일반적으로 이것은 가져온 댓글이 `Page ID`에 연결되어 있고 URL을 전달하는 경우(또는 값을 전달하지 않는 경우 기본적으로 페이지 URL이 사용됨)에 발생합니다.

[댓글을 내보내기](https://fastcomments.com/auth/my-account/manage-data/export)하고 `URL ID` 열(현재 열 `B`)을 확인하여 디버그할 수 있습니다.

`URL ID` 열에 표시되는 값이 위젯 구성에 `urlId` 매개변수로 전달하는 값과 동일한지 확인하세요.

자세한 설명은 [댓글이 페이지 및 기사에 연결되는 방법에 대한 문서](/guide-customizations-and-configuration.html#url-id)를 읽어보세요.

모든 방법이 실패하면 [저희에게 연락하세요](https://fastcomments.com/auth/my-account/help).

### 댓글 위젯이 표시되지 않음

댓글 위젯이 표시되지 않으면 Chrome 개발자 콘솔에서 오류를 확인하세요.

대부분의 잘못된 구성에서 댓글 위젯은 로드할 수 있는 경우 최소한 페이지에 오류를 표시합니다. 아무것도 보이지 않으면 일반적으로 스크립트 오류의 징후입니다.

### 원하는 구성이 예상대로 작동하지 않음

댓글 위젯에 전달되는 구성을 확인하려면 [Chrome 확장 프로그램](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)을 사용해 보세요. 모든 방법이 실패하면 Chrome 확장 프로그램이 표시하는 내용의 스크린샷을 찍어 [저희에게 연락하세요](https://fastcomments.com/auth/my-account/help).

### 다른 해시뱅을 가진 동일한 URL에서 댓글이 누락됨

기본적으로 FastComments는 댓글이 저장되는 "버킷"으로 페이지 URL을 사용합니다. URL에 `#hashbangs`가 포함되어 있고 이러한 `#hashbangs`가 댓글 스레드를 식별하는 식별자의 일부가 아니어야 하는 경우 해시뱅 값을 단순히 무시할 수 있습니다. 예:

[inline-code-attrs-start title = '해시뱅 무시 예제'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

이 변경을 한 후에는 기존 댓글에 대한 마이그레이션을 수행해야 합니다. [그것에 대해서는 저희에게 연락하세요.](https://fastcomments.com/auth/my-account/help)

### URL 쿼리 매개변수가 위젯에 영향을 미침

기본적으로 FastComments는 댓글이 저장되는 "버킷"으로 페이지 URL을 사용합니다. URL에 댓글 스레드를 식별하는 식별자의 일부가 아니어야 하는 쿼리 매개변수가 포함되어 있는 경우 단순히 무시할 수 있습니다. 예:

[inline-code-attrs-start title = '쿼리 매개변수 무시'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

이 변경을 한 후에는 기존 댓글에 대한 마이그레이션을 수행해야 합니다. [그것에 대해서는 저희에게 연락하세요.](https://fastcomments.com/auth/my-account/help)

### 이메일을 받지 못함

FastComments에서는 이메일 전달이 가능한 한 안정적이 되도록 많은 노력을 기울입니다. 그러나 일부 이메일 제공업체는 안정적으로 전달하기가 매우 어려운 것으로 알려져 있습니다. fastcomments.com에서 보낸 메시지가 있는지 스팸 폴더를 확인하세요.

[저희에게 연락](https://fastcomments.com/auth/my-account/help)하시면 일반적으로 저희 이메일을 받지 못하는 이유에 대해 더 많은 정보를 제공할 수 있습니다.
