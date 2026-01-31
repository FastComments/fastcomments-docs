다음은 자주 발생하는 몇 가지 증상과 일반적인 해결책입니다. 

### "This is a demo" 메시지

이 메시지는 홈페이지에서 위젯 코드를 복사해 사용했을 때 표시되며, 홈페이지에서는 데모 테넌트를 사용합니다. 자신의 테넌트를 사용하려면 [여기](https://fastcomments.com/auth/my-account/get-acct-code)에서 위젯 코드를 복사하세요.

### "FastComments cannot load on this domain" 오류

FastComments는 계정과 연관된 요청을 인증하기 위해 어떤 도메인이 귀하 소유인지 알아야 합니다. 이 오류를 해결하는 방법은 [문서](/guide-multiple-sites.html#add-domains-to-account)를 확인하세요 (정확한 서브도메인 + 도메인을 계정에 추가하면 됩니다).

이 오류는 체험 기간이 끝난 이후에만 발생한다는 점에 유의하세요. 체험 기간 동안에는 새 도메인에서 오는 요청이 자동으로 계정에 추가됩니다.

### 마이그레이션된 댓글이 커스텀 설치에서 표시되지 않는 경우

보통 이는 가져온 댓글이 `Page ID`에 연결되어 있는데, 위젯에 URL을 전달하고 있거나 (값을 전달하지 않으면 기본적으로 페이지 URL이 사용됩니다) 하는 경우에 발생합니다.

이 문제는 [댓글을 내보내기](https://fastcomments.com/auth/my-account/manage-data/export)하여 `URL ID` 열(현재 열 `B`)을 확인하면 디버그할 수 있습니다.

`URL ID` 열에 보이는 값이 위젯 구성에서 `urlId` 매개변수로 전달하고 있는 값과 동일한지 확인하세요.

자세한 설명은 [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id)를 참조해 보세요.

그래도 해결되지 않으면, [문의해 주세요](https://fastcomments.com/auth/my-account/help).

### 댓글 위젯이 표시되지 않는 경우

댓글 위젯이 표시되지 않는 경우 Chrome 개발자 콘솔에서 오류를 확인하세요.

대부분의 잘못된 구성에서는 댓글 위젯이 로드될 수 있다면 적어도 페이지에 오류를 표시합니다. 아무것도 보이지 않는 것은 보통 스크립트 오류가 있다는 신호입니다.

### 원하는 구성대로 작동하지 않는 경우

댓글 위젯에 전달된 구성을 확인하려면 [Chrome 확장 프로그램](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)을 사용해 보세요. 그래도 해결되지 않으면, Chrome 확장 프로그램이 보여주는 내용을 스크린샷으로 찍어 [문의해 주세요](https://fastcomments.com/auth/my-account/help).

### 서로 다른 해시뱅으로 동일한 URL에서 댓글이 누락되는 경우

기본적으로, FastComments는 댓글이 저장되는 "버킷"으로 페이지 URL을 사용합니다. URL에 `#hashbangs`가 포함되어 있고 이 `#hashbangs`가 댓글 스레드를 식별하는 식별자의 일부가 되어서는 안 되는 경우, 해시뱅 값을 단순히 무시할 수 있습니다. 예:

[inline-code-attrs-start title = '해시뱅 무시 예제'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

이 변경을 적용한 후에는 기존 댓글에 대해 마이그레이션을 수행해야 합니다. [이를 위해 문의해 주세요.](https://fastcomments.com/auth/my-account/help)

### URL 쿼리 매개변수가 위젯에 영향을 줄 때

기본적으로, FastComments는 댓글이 저장되는 "버킷"으로 페이지 URL을 사용합니다. URL에 댓글 스레드를 식별하는 식별자의 일부가 되어서는 안 되는 쿼리 매개변수가 포함되어 있는 경우, 해당 쿼리 매개변수를 단순히 무시할 수 있습니다. 예:

[inline-code-attrs-start title = '쿼리 매개변수 무시'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

이 변경을 적용한 후에는 기존 댓글에 대해 마이그레이션을 수행해야 합니다. [이를 위해 문의해 주세요.](https://fastcomments.com/auth/my-account/help)

### 이메일을 받지 못하는 경우

FastComments에서는 이메일 전송의 신뢰성을 최대한 보장하기 위해 많은 노력을 기울이고 있습니다. 그러나 일부 이메일 제공업체는 신뢰성 있게 전달하기가 특히 어렵습니다. fastcomments.com에서 온 메시지가 스팸 폴더로 분류되지 않았는지 확인하세요.

[문의해 주시면](https://fastcomments.com/auth/my-account/help) 저희가 이메일을 받지 못하는 원인에 대해 더 자세한 정보를 제공해 드릴 수 있습니다.