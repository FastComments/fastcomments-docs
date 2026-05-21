Jeśli nie możesz zainstalować [aplikacji Shopify App Store](https://apps.shopify.com/fastcomments), nadal możesz dodać FastComments, edytując swój motyw. Ta ścieżka jest przydatna, gdy chcesz podłączyć tenant FastComments, który już posiadasz, lub gdy osadzasz komentarze na sklepie Shopify, gdzie aplikacja nie jest dostępna.

Instalacja przez aplikację to obsługiwana ścieżka dla większości sklepów. Skorzystaj z niej tylko, jeśli aplikacja nie pasuje do Twojego przypadku.

### Krok 1: Wyłącz natywne komentarze Shopify

W panelu administracyjnym Shopify przejdź do **Blog posts > Manage blogs**, otwórz każdy blog i ustaw **Comments are disabled** w panelu po prawej stronie. Zapisz.

To zapobiega wyświetlaniu się wbudowanego systemu komentarzy Shopify obok FastComments.

### Krok 2: Otwórz szablon artykułu blogowego

W panelu administracyjnym Shopify:

1. Przejdź do **Online Store > Themes**.
2. Pod swoim aktualnym motywem kliknij **Actions > Edit code**.
3. W przeglądarce plików po lewej otwórz **Sections** i kliknij `main-article.liquid`.

To jest szablon, który Shopify renderuje dla pojedynczego artykułu blogowego.

### Krok 3: Wklej snippet FastComments

Przewiń mniej więcej do linii 100 pliku `main-article.liquid`, tuż po zamykającym `</div>` treści artykułu. Wklej następujący snippet:

[inline-code-attrs-start title = 'Fragment FastComments dla Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Zamień `"demo"` na własny Tenant ID z [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Kliknij **Zapisz**.

### Krok 4: Autoryzuj domenę sklepu

Otwórz wpis na blogu na swoim sklepie na żywo. Jeśli zamiast widżetu komentarzy zobaczysz błąd autoryzacji, FastComments musi wiedzieć, że Twój sklep może korzystać z tego tenantu. Zobacz [Błędy domeny](/guide-installation-shopify.html#shopify-domain-errors).

### Dodawanie FastComments do innych stron

Ten sam snippet działa w dowolnym szablonie Liquid, w tym na stronach produktu, stronach niestandardowych i na stronie głównej. Wklej go tam, gdzie chcesz, aby pojawiały się komentarze i dostosuj `urlId`, jeśli chcesz mieć stabilny identyfikator dla każdej strony (na przykład, `urlId: "{{ product.id }}"` w szablonie produktu).