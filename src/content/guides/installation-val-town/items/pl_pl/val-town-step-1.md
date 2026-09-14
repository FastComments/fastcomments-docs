The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Widżet komentarzy w HTTP val'; type='javascript' inline-code-attrs-end]
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

## Wybierz urlId przed wypuszczeniem

`urlId` decyduje, w którym wątku pojawi się komentarz. Jeśli pozostawisz go nieustawionym, domyślnie przyjmuje wyczyszczoną wersję bieżącego adresu URL strony, co jest dokładnie tym, co zmienia się w Val Town: val ma długą nazwę hosta `*.web.val.run`, dopóki nie zarezerwujesz subdomeny, gałęzie otrzymują własne adresy URL, a zmiana nazwy strony zmienia ścieżkę. Każda wariacja cicho staje się osobnym, pustym wątkiem, a objaw wygląda jak „moje komentarze zniknęły”.

Ustaw go na coś stabilnego, co kontrolujesz, np. slug posta lub identyfikator w bazie danych, jak powyżej. Przekaż także `url`, aby e‑maile z powiadomieniami i narzędzia moderacji mogły odwoływać się do rzeczywistej strony.

## Zachowanie komentarzy bez JavaScriptu

FastComments renderuje pełny wątek po stronie serwera, który val może wstawić do bloku `<noscript>`:

[inline-code-attrs-start title = 'Zapasowy tryb bez JavaScriptu'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Zakoduj parametry w formacie URL. Wersja po stronie serwera obsługuje komentowanie anonimowe i zalogowane, SSO oraz zagnieżdżone odpowiedzi.