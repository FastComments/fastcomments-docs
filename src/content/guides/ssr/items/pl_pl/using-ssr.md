Aby użyć FastComments SSR, klient może pobrać HTML z punktu końcowego `https://fastcomments.com/ssr/comments`.

Można to zrobić na kilka sposobów.

### Z WordPressem

SSR jest domyślnie włączony jako rozwiązanie zapasowe dla użytkowników bez włączonego JS w wtyczce WordPress od wersji `3.10.2`.

### Na stronie internetowej

W istniejącej aplikacji SSR można dodać przy użyciu [następującego przykładu](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), zakładając, że używany język to PHP:

[inline-code-attrs-start title = 'Przykład SSR oparty na PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Możemy również wyświetlić interfejs SSR tylko wtedy, gdy użytkownik ma wyłączony JS:

[inline-code-attrs-start title = 'Przykład zapasowy SSR oparty na PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Aby zobaczyć przykład użycia SSO, [zobacz tutaj](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Z wstępnie renderowaną treścią

Nasz blog jest generowany podczas procesu budowania i zawiera [dobry przykład SSR z użyciem Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Podstawowe parametry

Podstawowe parametry, które należy przekazać, to:
- `tenantId` - To identyfikuje Cię jako klienta.
- `urlId` - To identyfikuje stronę lub artykuł, dla którego mają być załadowane komentarze, i określa miejsce, gdzie są zapisywane.
- `url` - Służy do powiadomień i powiązanych funkcji, by odnieść się z powrotem do wątku komentarzy.

### Niestandardowe style

Wersja SSR widżetu komentarzy używa tej samej struktury i silnika renderującego co wersja JavaScript.

W związku z tym wszystkie niestandardowe style działające dla widżetu komentarzy w JavaScript działają także dla SSR. 

### Uwagi

W przypadku SSR nie ma JavaScriptu, który kontrolowałby wysokość renderowanego kontenera. W przeglądarkach dla długich dyskusji może pojawić się pionowy pasek przewijania.

W związku z tym należy to dostosować według potrzeb.