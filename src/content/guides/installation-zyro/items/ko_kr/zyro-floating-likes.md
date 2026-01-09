---
FastComments는 Zyro용 페이지 리액트(일명 플로팅 좋아요 버튼) 위젯도 지원합니다.

이 페이지 오른쪽 하단에서 작동하는 모습을 볼 수 있습니다!

1. 먼저, 코드를 가져옵니다:

[inline-code-attrs-start title = 'Zyro 플로팅 좋아요 코드'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. 그런 다음 Zyro에서 사이트 빌더를 엽니다.
3. 왼쪽 하단의 Website Settings로 이동합니다.
4. Integrations를 선택합니다.
5. 새 코드를 `Custom code` 필드의 *끝*에 추가하고 사이트를 게시합니다.
6. 미리보기 모드에서는 위젯이 보이지 않지만, 사이트의 게시된 버전에서는 나타납니다.

---