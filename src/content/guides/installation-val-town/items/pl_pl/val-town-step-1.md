Widżet to tag skryptu i element kontenera, więc pasuje do tego, co Twoja val już renderuje. Ten przykład używa Hono JSX, czyli tego, czego używają szablony HTTP Val Town.

[inline-code-attrs-start title = 'Widżet komentarzy w val HTTP'; type='javascript' inline-code-attrs-end]
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

## Wybierz urlId przed publikacją

`urlId` decyduje o tym, do którego wątku trafi komentarz. Jeśli go nie ustawisz, domyślnie będzie to oczyszczona wersja adresu URL bieżącej strony — a to dokładnie to, co zmienia się w Val Town: val ma długi hostname `*.web.val.run`, dopóki nie zajmiesz subdomeny, gałęzie mają własne adresy URL, a zmiana nazwy strony zmienia ścieżkę. Każda wariacja po cichu staje się osobnym, pustym wątkiem, a objaw wygląda jak „moje komentarze zniknęły”.

Ustaw coś stabilnego, czym sterujesz, np. slug posta lub identyfikator z bazy danych, jak powyżej. Przekaż też `url`, aby e-maile z powiadomieniami i narzędzia moderacji mogły linkować z powrotem do prawdziwej strony.

## Zachowywanie komentarzy bez JavaScript

FastComments renderuje cały wątek po stronie serwera, a val może wstawić go do bloku `<noscript>`:

[inline-code-attrs-start title = 'Rozwiązanie zastępcze bez JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Zakoduj parametry w URL. Wersja po stronie serwera obsługuje komentowanie anonimowe i zalogowane, SSO oraz zagnieżdżone odpowiedzi.