Widget bir script etiketi ve bir konteyner elemanından oluşur, bu nedenle val'inizin zaten render ettiği her şeye doğrudan yerleştirilebilir. Bu örnek, Val Town'un HTTP şablonlarının kullandığı Hono JSX'i kullanır.

[inline-code-attrs-start title = 'HTTP val içinde yorum widget\'ı'; type='javascript' inline-code-attrs-end]
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

## Yayınlamadan önce bir urlId seçin

`urlId`, bir yorumun hangi konuya düşeceğini belirler. Ayarlanmamış bırakılırsa, geçerli sayfa URL'sinin temizlenmiş bir sürümüne varsayılan olur — bu da tam olarak Val Town'da değişen şeydir: bir val, bir alt alan adı talep edene kadar uzun bir `*.web.val.run` ana makine adına sahiptir, dallar kendi URL'lerini alır ve bir sayfayı yeniden adlandırmak yolu değiştirir. Her varyasyon sessizce ayrı ve boş bir konu haline gelir ve belirti "yorumlarım kayboldu" şeklinde okunur.

Yukarıdaki gibi, kontrol ettiğiniz sabit bir değere ayarlayın; örneğin yazı slug'ı veya bir veritabanı kimliği. Ayrıca `url` değerini de iletin, böylece bildirim e-postaları ve moderasyon araçları gerçek sayfaya bağlantı verebilir.

## JavaScript olmadan yorumları korumak

FastComments, tam bir konuyu sunucu tarafında render eder; bir val bunu bir `<noscript>` bloğuna yerleştirebilir:

[inline-code-attrs-start title = 'JavaScript olmadan yedek'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Parametreleri URL-encode edin. Sunucu tarafı sürüm, anonim ve oturum açmış yorum yapmayı, SSO'yu ve iç içe yanıtları destekler.