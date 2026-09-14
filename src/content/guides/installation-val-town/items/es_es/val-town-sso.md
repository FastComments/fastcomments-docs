If your val already knows who the visitor is, Secure SSO hands that identity to the widget so they never see a second login. There are no endpoints to build and nothing to call at runtime: you compute three values server-side and pass them in the widget config.

Val Town ships zero-config login with `std/oauth`, so the visitor can sign in with the Val Town account they already have. Swap that for whatever your app uses; the FastComments half does not change.

## Build the payload on the server

The API secret signs the payload and must never reach browser code. Install the SDK from npm, which works on Val Town's Deno runtime as-is:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // el id debe ser estable para la misma persona, o recibirán una nueva identidad de comentario en cada inicio de sesión.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // el correo electrónico es obligatorio y debe ser único.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // el nombre de usuario es obligatorio y no puede ser un correo electrónico.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` returns `{ userDataJSONBase64, verificationHash, timestamp }`. Those three values are all that reach the browser. The secret signs them and is then dropped, so nothing in the page lets a reader forge a different user.

## Pass it to the widget

[inline-code-attrs-start title = 'Configuración del widget con SSO'; type='javascript' inline-code-attrs-end]
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

  // ...renderizar el widget con esta configuración
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` adds `GET /auth/login`, `GET /auth/callback` and `POST /auth/logout` for you. Note that logout is a **POST**, while the widget navigates to `logoutURL` with a GET, so point `logoutURL` at a small route of your own that submits the POST.

When the visitor is logged out, pass `sso` with only a `loginURL`. The widget then shows a login prompt instead of an anonymous comment box.

## Things that go wrong

`timestamp` is epoch **milliseconds**, must not be in the future, and must not be more than two days old. Generate it on the server in the same request that computes the hash. Generating it in the browser is the classic failure: the value differs from the one that was hashed and every comment is rejected.

Never set `isAdmin` or `isModerator` from the identity provider. Signing in with a Val Town account says nothing about who should moderate your site.

See the [SSO guide](/guide-sso.html) for the full field list, group-gated threads, and badges.