이미 방문자가 누구인지 val이 알고 있다면, Secure SSO는 해당 신원을 위젯에 전달하여 두 번째 로그인을 보지 않도록 합니다. 만들 엔드포인트도 없고 런타임에 호출할 것도 없습니다: 서버 측에서 세 개의 값을 계산하여 위젯 설정에 전달하면 됩니다.

Val Town은 `std/oauth`를 통해 설정 없이 바로 사용할 수 있는 로그인을 제공하므로, 방문자는 이미 가지고 있는 Val Town 계정으로 로그인할 수 있습니다. 이를 여러분의 앱에서 사용하는 것으로 바꿔도 FastComments 쪽은 변경되지 않습니다.

## 서버에서 페이로드 빌드하기

API 시크릿은 페이로드에 서명하며 브라우저 코드에 절대 노출되어서는 안 됩니다. npm에서 SDK를 설치하세요. Val Town의 Deno 런타임에서 그대로 작동합니다:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id must be stable for the same person, or they get a new comment identity on every login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is required and must be unique.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is required and cannot be an email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()`는 `{ userDataJSONBase64, verificationHash, timestamp }`를 반환합니다. 이 세 값만 브라우저에 전달됩니다. 시크릿은 이들에 서명한 후 폐기되므로, 페이지의 어떤 것도 다른 사용자를 위조할 수 없습니다.

## 위젯에 전달하기

[inline-code-attrs-start title = 'SSO가 포함된 위젯 설정'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware`는 `GET /auth/login`, `GET /auth/callback`, `POST /auth/logout`을 자동으로 추가해 줍니다. 로그아웃은 **POST**인 반면, 위젯은 `logoutURL`을 GET으로 이동하므로, `logoutURL`이 POST를 전송하는 여러분만의 작은 라우트를 가리키도록 하세요.

방문자가 로그아웃 상태일 때는 `loginURL`만 포함된 `sso`를 전달하세요. 그러면 위젯은 익명 댓글 상자 대신 로그인 프롬프트를 표시합니다.

## 자주 발생하는 문제

`timestamp`는 에포크 **밀리초** 단위여야 하고, 미래 시점이어서는 안 되며, 이틀 이상 오래되어서도 안 됩니다. 해시를 계산하는 동일한 요청에서 서버에서 생성하세요. 브라우저에서 생성하는 것이 전형적인 실패 사례입니다: 값이 해시된 값과 달라져 모든 댓글이 거부됩니다.

신원 제공자(identity provider)에서 `isAdmin`이나 `isModerator`를 절대 설정하지 마세요. Val Town 계정으로 로그인하는 것은 누가 여러분의 사이트를 중재해야 하는지에 대한 아무런 정보도 제공하지 않습니다.

전체 필드 목록, 그룹 기반 스레드, 배지에 대해서는 [SSO 가이드](/guide-customizations-and-configuration.html#sso-secure)를 참조하세요.