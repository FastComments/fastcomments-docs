Se o seu val já sabe quem é o visitante, o Secure SSO entrega essa identidade ao widget para que ele nunca veja um segundo login. Não há endpoints para construir e nada para chamar em tempo de execução: você calcula três valores no servidor e os passa na configuração do widget.

O Val Town oferece login sem configuração com `std/oauth`, então o visitante pode entrar com a conta Val Town que já possui. Troque isso por qualquer autenticação que seu app use; a parte do FastComments não muda.

## Construa o payload no servidor

O segredo da API assina o payload e nunca deve chegar ao código do navegador. Instale o SDK do npm, que funciona no runtime Deno do Val Town como está:

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

`getPayload()` retorna `{ userDataJSONBase64, verificationHash, timestamp }`. Esses três valores são tudo o que chega ao navegador. O segredo os assina e é então descartado, então nada na página permite que um leitor falsifique um usuário diferente.

## Passe-o ao widget

[inline-code-attrs-start title = 'Configuração do widget com SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` adiciona `GET /auth/login`, `GET /auth/callback` e `POST /auth/logout` para você. Note que o logout é um **POST**, enquanto o widget navega para `logoutURL` com um GET, então aponte `logoutURL` para uma pequena rota sua que submeta o POST.

Quando o visitante está deslogado, passe `sso` com apenas um `loginURL`. O widget então exibirá um prompt de login em vez de uma caixa de comentários anônima.

## Coisas que dão errado

`timestamp` está em **milissegundos** de época, não deve estar no futuro e não deve ter mais de dois dias. Gere-o no servidor na mesma requisição que calcula o hash. Gerá-lo no navegador é a falha clássica: o valor difere daquele que foi assinado e todo comentário é rejeitado.

Nunca defina `isAdmin` ou `isModerator` a partir do provedor de identidade. Entrar com uma conta Val Town não diz nada sobre quem deve moderar seu site.

Consulte o [guia de SSO](/guide-customizations-and-configuration.html#sso-secure) para a lista completa de campos, threads restritas a grupos e badges.