위젯은 script 태그와 컨테이너 요소로 구성되어 있으므로, val이 이미 렌더링하는 무엇이든 손쉽게 삽입할 수 있습니다. 이 예제는 Val Town의 HTTP 템플릿에서 사용하는 Hono JSX를 사용합니다.

[inline-code-attrs-start title = 'HTTP val의 댓글 위젯'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## 배포 전에 urlId를 정하세요

`urlId`는 댓글이 어느 스레드에 속하는지 결정합니다. 설정하지 않으면 현재 페이지 URL을 정리한 버전이 기본값으로 사용되는데, 이것이 바로 Val Town에서 변경되는 부분입니다: 서브도메인을 등록하기 전까지 val은 긴 `*.web.val.run` 호스트명을 가지며, 브랜치마다 별도의 URL이 생기고, 페이지 이름을 바꾸면 경로가 달라집니다. 이러한 변화마다 조용히 별개의 빈 스레드가 만들어지고, 그 결과 "내 댓글이 사라졌다"는 증상으로 나타납니다.

위 예제처럼 게시글 슬러그나 데이터베이스 id처럼 여러분이 관리하는 안정적인 값으로 설정하세요. `url`도 함께 전달하면 알림 이메일과 관리 도구가 실제 페이지로 연결될 수 있습니다.

## JavaScript 없이 댓글 유지하기

FastComments는 전체 스레드를 서버 측에서 렌더링하므로, val에서 `<noscript>` 블록에 넣을 수 있습니다:

[inline-code-attrs-start title = 'JavaScript 없는 대체 방식'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

파라미터는 URL 인코딩하세요. 서버 측 버전은 익명 및 로그인 댓글 작성, SSO, 중첩 답글을 지원합니다.