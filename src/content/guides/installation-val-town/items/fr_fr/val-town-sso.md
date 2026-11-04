Si votre val connaît déjà l'identité du visiteur, Secure SSO transmet cette identité au widget afin qu'il ne voie jamais un second écran de connexion. Il n'y a aucun endpoint à construire et rien à appeler à l'exécution : vous calculez trois valeurs côté serveur et les passez dans la configuration du widget.

Val Town propose une connexion sans configuration avec `std/oauth`, afin que le visiteur puisse se connecter avec le compte Val Town qu'il possède déjà. Remplacez cela par ce que votre application utilise ; la partie FastComments ne change pas.

## Construire le payload côté serveur

Le secret d'API signe le payload et ne doit jamais atteindre le code du navigateur. Installez le SDK depuis npm, qui fonctionne tel quel sur le runtime Deno de Val Town :

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

`getPayload()` renvoie `{ userDataJSONBase64, verificationHash, timestamp }`. Ces trois valeurs sont tout ce qui atteint le navigateur. Le secret les signe puis est abandonné, de sorte que rien dans la page ne permet à un lecteur de falsifier un autre utilisateur.

## Le passer au widget

[inline-code-attrs-start title = 'Configuration du widget avec SSO'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` ajoute `GET /auth/login`, `GET /auth/callback` et `POST /auth/logout` pour vous. Notez que la déconnexion est un **POST**, tandis que le widget navigue vers `logoutURL` avec un GET ; faites donc pointer `logoutURL` vers une petite route de votre cru qui soumet le POST.

Lorsque le visiteur est déconnecté, passez `sso` avec uniquement un `loginURL`. Le widget affiche alors une invite de connexion au lieu d'une boîte de commentaires anonyme.

## Ce qui peut mal tourner

`timestamp` est en **millisecondes** epoch, ne doit pas être dans le futur et ne doit pas dater de plus de deux jours. Générez-le côté serveur dans la même requête que celle qui calcule le hash. Le générer dans le navigateur est l'erreur classique : la valeur diffère de celle qui a été signée et chaque commentaire est rejeté.

Ne définissez jamais `isAdmin` ou `isModerator` à partir du fournisseur d'identité. Se connecter avec un compte Val Town ne dit rien de qui devrait modérer votre site.

Consultez le [guide SSO](/guide-customizations-and-configuration.html#sso-secure) pour la liste complète des champs, les fils restreints par groupe et les badges.