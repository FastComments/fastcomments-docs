Poniżej znajduje się kod Vanilla JS do instalacji widżetu Podsumowania. Biblioteka React również posiada ten widżet.

[inline-code-attrs-start title = 'Instalacja widżetu Podsumowania'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widżet automatycznie znajdzie pytania do wyświetlenia w podsumowaniu na podstawie odpowiedniej konfiguracji widżetu dla danej strony/witryny.

### Filtrowanie komentarzy według oceny

Gdy widżet komentarzy dla tej samej strony znajduje się również na stronie i jest skonfigurowany tak, aby wyświetlać odpowiedź każdego komentującego przy jego komentarzu, wiersze podziału
stają się klikalne. Kliknięcie wiersza, na przykład "5", filtruje widżet komentarzy do komentarzy, które udzieliły tej odpowiedzi. Ponowne kliknięcie wiersza lub X na plakietce "Wyświetlanie tylko"
w widżecie komentarzy ponownie pokazuje wszystkie komentarze. Nie wymaga to żadnej konfiguracji: Widżet Podsumowania znajduje widżet komentarzy z tym samym `tenantId` i `urlId`.

Działa to w przypadku pytań typu gwiazdki, kciuki, emoji, skala i pojedynczy wybór. Podział wyświetlany przez widżet komentarzy nad polem komentarza można klikać w ten sam sposób.

Jeśli potrzebujesz tego widżetu w jednej z naszych innych bibliotek, która go nie posiada, zgłoś zgłoszenie do wsparcia, abyśmy wiedzieli, że należy go dodać.