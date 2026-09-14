The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

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

## 배포 전에 urlId 선택하기

`urlId`는 댓글이 어느 스레드에 달릴지를 결정합니다. 이를 설정하지 않으면 현재 페이지 URL을 정리한 버전이 기본값으로 사용되는데, 이는 Val Town에서 변하는 바로 그 요소입니다: 서브도메인을 획득하기 전까지 val은 긴 `*.web.val.run` 호스트명을 가지고, 브랜치는 각각 고유한 URL을 가지며, 페이지 이름을 바꾸면 경로가 바뀝니다. 각 변형은 조용히 별도의 빈 스레드가 되며, 그 결과는 “내 댓글이 사라졌다”는 현상으로 나타납니다.

위와 같이 게시물 슬러그나 데이터베이스 ID와 같이 여러분이 제어할 수 있는 안정적인 값으로 설정하세요. 또한 `url`을 전달하면 알림 이메일 및 관리 도구가 실제 페이지로 연결될 수 있습니다.

## JavaScript 없이 댓글 유지하기

FastComments는 전체 스레드를 서버 측에서 렌더링하므로, val은 이를 `<noscript>` 블록에 삽입할 수 있습니다:

[inline-code-attrs-start title = 'JavaScript 없는 대체 방법'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

매개변수를 URL 인코딩하세요. 서버 측 버전은 익명 및 로그인 댓글, SSO, 그리고 중첩 답글을 지원합니다.