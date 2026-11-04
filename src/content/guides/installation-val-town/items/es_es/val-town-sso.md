Si tu val ya sabe quién es el visitante, Secure SSO le pasa esa identidad al widget para que nunca vea un segundo inicio de sesión. No hay endpoints que construir ni nada que llamar en tiempo de ejecución: calculas tres valores en el servidor y los pasas en la configuración del widget.

Val Town incluye inicio de sesión sin configuración con `std/oauth`, así que el visitante puede iniciar sesión con la cuenta de Val Town que ya tiene. Cámbialo por el que use tu aplicación; la parte de FastComments no cambia.

## Construye el payload en el servidor

El API secret firma el payload y nunca debe llegar al código del navegador. Instala el SDK desde npm, que funciona en el runtime Deno de Val Town tal cual:

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

`getPayload()` devuelve `{ userDataJSONBase64, verificationHash, timestamp }`. Esos tres valores son todo lo que llega al navegador. El secret los firma y luego se descarta, así que nada en la página permite a un lector suplantar a otro usuario.

## Pásalo al widget

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

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` añade `GET /auth/login`, `GET /auth/callback` y `POST /auth/logout` por ti. Ten en cuenta que el logout es un **POST**, mientras que el widget navega a `logoutURL` con un GET, así que apunta `logoutURL` a una pequeña ruta propia que envíe el POST.

Cuando el visitante ha cerrado sesión, pasa `sso` con solo un `loginURL`. El widget mostrará entonces un mensaje de inicio de sesión en lugar de un cuadro de comentarios anónimo.

## Cosas que salen mal

`timestamp` está en **milisegundos** de época, no debe estar en el futuro y no debe tener más de dos días de antigüedad. Genéralo en el servidor en la misma petición que calcula el hash. Generarlo en el navegador es el fallo clásico: el valor difiere del que se firmó con el hash y cada comentario es rechazado.

Nunca establezcas `isAdmin` o `isModerator` a partir del proveedor de identidad. Iniciar sesión con una cuenta de Val Town no dice nada sobre quién debería moderar tu sitio.

Consulta la [guía de SSO](/guide-customizations-and-configuration.html#sso-secure) para ver la lista completa de campos, los hilos restringidos por grupos y las insignias.