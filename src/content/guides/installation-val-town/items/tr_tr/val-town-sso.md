Val'iniz ziyaretçinin kim olduğunu zaten biliyorsa, Secure SSO bu kimliği widget'a aktarır ve ziyaretçi ikinci bir giriş ekranı görmez. Oluşturulacak bir uç nokta yok ve çalışma zamanında çağrılacak bir şey yok: üç değeri sunucu tarafında hesaplar ve widget yapılandırmasına geçirirsiniz.

Val Town, `std/oauth` ile sıfır yapılandırmalı giriş sunar, böylece ziyaretçi zaten sahip olduğu Val Town hesabıyla oturum açabilir. Bunu uygulamanızın kullandığı herhangi bir şeyle değiştirebilirsiniz; FastComments tarafı değişmez.

## Payload'ı sunucuda oluşturun

API gizli anahtarı payload'ı imzalar ve asla tarayıcı koduna ulaşmamalıdır. SDK'yı npm'den yükleyin; Val Town'un Deno çalışma zamanında olduğu gibi çalışır:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id aynı kişi için sabit olmalıdır, aksi halde her girişte yeni bir yorum kimliği alır.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email zorunludur ve benzersiz olmalıdır.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username zorunludur ve bir e-posta olamaz.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()`, `{ userDataJSONBase64, verificationHash, timestamp }` değerlerini döndürür. Tarayıcıya ulaşan yalnızca bu üç değerdir. Gizli anahtar bunları imzalar ve ardından atılır, böylece sayfadaki hiçbir şey bir okuyucunun farklı bir kullanıcı taklit etmesini sağlamaz.

## Widget'a geçirin

[inline-code-attrs-start title = 'SSO ile widget yapılandırması'; type='javascript' inline-code-attrs-end]
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

  // ...widget'ı bu yapılandırmayla render edin
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware`, sizin için `GET /auth/login`, `GET /auth/callback` ve `POST /auth/logout` uç noktalarını ekler. Çıkış işleminin bir **POST** olduğunu, ancak widget'ın `logoutURL` adresine GET ile gittiğini unutmayın; bu nedenle `logoutURL` adresini, POST isteğini gönderen kendi küçük bir rotanıza yönlendirin.

Ziyaretçi oturum açmamışken, yalnızca bir `loginURL` içeren `sso` değerini geçirin. Widget daha sonra anonim yorum kutusu yerine bir giriş istemi gösterir.

## Ters gidebilecek şeyler

`timestamp` epoch **milisaniyesi** olmalıdır, gelecekte olmamalı ve iki günden eski olmamalıdır. Hash'i hesaplayan aynı istekte sunucu tarafında oluşturun. Bunu tarayıcıda oluşturmak klasik hatadır: değer, hash'lenenden farklı olur ve her yorum reddedilir.

Kimlik sağlayıcıdan asla `isAdmin` veya `isModerator` ayarlamayın. Bir Val Town hesabıyla oturum açmak, sitenizi kimin yönetmesi gerektiği hakkında hiçbir şey söylemez.

Alan listesinin tamamı, grup korumalı konular ve rozetler için [SSO rehberine](/guide-customizations-and-configuration.html#sso-secure) bakın.