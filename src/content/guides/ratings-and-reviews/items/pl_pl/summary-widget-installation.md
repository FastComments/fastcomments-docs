---
Poniżej znajduje się kod Vanilla JS do instalacji widżetu podsumowania. Biblioteka React również zawiera ten widżet.

[inline-code-attrs-start title = 'Instalacja widżetu podsumowania'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widżet automatycznie znajdzie pytania, które mają być pokazane w podsumowaniu, w oparciu o odpowiednią konfigurację widżetu dla tej strony/witryny.

Jeśli potrzebujesz tego widżetu w jednej z naszych innych bibliotek, która go nie zawiera, otwórz zgłoszenie do wsparcia, abyśmy wiedzieli, że mamy go dodać.

---