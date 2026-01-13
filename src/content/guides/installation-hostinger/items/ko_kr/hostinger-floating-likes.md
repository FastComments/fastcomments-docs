---
FastComments는 Hostinger용 Page Reacts(일명 플로팅 좋아요 버튼) 위젯도 지원합니다.

이 페이지의 오른쪽 하단에서 작동 모습을 확인할 수 있습니다!

### 참고!

이 지침은 Hostinger 사이트 빌더용입니다. Hostinger *WordPress*를 사용 중이라면, 아래 코드를 복사하여 WordPress 사이트에 추가할 때 [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/)를 사용하세요. WPCode는 사이트에 작은 코드 스니펫을 추가하기 위한 무료이자 사용하기 쉬운 플러그인입니다.

1. 먼저, 코드를 가져오세요:

[inline-code-attrs-start title = 'Hostinger 플로팅 좋아요 코드'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. 그런 다음 Hostinger에서 사이트 빌더를 엽니다.
3. 왼쪽 하단의 Website Settings(웹사이트 설정)로 이동합니다.
4. Integrations(통합)를 선택합니다.
5. 새 코드를 `Custom code` 필드의 *끝*에 추가하고 사이트를 게시합니다.
6. 미리보기 모드에서는 위젯이 보이지 않지만, 사이트의 게시된 버전에서는 나타납니다.

---