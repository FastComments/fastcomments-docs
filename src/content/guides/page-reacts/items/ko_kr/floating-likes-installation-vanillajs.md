설치는 간단합니다:

[inline-code-attrs-start title = '플로팅 좋아요 코드 예제'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

생성자의 타입 시그니처는 다음과 같습니다:

[inline-code-attrs-start title = '구성'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - 페이지/기사 ID를 설정하려면 이를 변경하세요. 기본적으로 페이지 URL입니다.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

이는 인증을 위해 반응(좋아요 등)을 사용자의 계정에 연결하는 SSO를 지원합니다.

현재는 VanillaJS만 지원됩니다. 이 위젯을 당사의 클라이언트 라이브러리 중 하나에 추가하길 원하시면 알려주세요!

### 비동기 버전

[inline-code-attrs-start title = '플로팅 좋아요 코드 비동기 예제'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: 'demo'
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

### SSO 사용

보안 SSO(Secure SSO) 또는 간단한 SSO(Simple SSO) 페이로드도 전달할 수 있습니다:

[inline-code-attrs-start title = '플로팅 좋아요 보안 SSO 코드 예제'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // sso 객체 전달
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = '플로팅 좋아요 간단한 SSO 코드 예제'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        simpleSSO: {
            id: 'some-user-id',
            username: 'some username',
            email: 'some@email.com',
        }
    });
</script>
[inline-code-end]

---