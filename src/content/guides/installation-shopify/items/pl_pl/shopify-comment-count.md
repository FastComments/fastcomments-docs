Blok **FastComments - Comment Count** wyświetla mały licznik komentarzy dla pojedynczej strony. Użyj go na listach wpisów na blogu, kartach produktów lub w dowolnym szablonie, który prowadzi do strony z komentarzami, aby odwiedzający mogli zobaczyć, jak aktywny jest każdy wątek, zanim w niego klikną.

### Add the block

1. Otwórz edytor motywu Shopify.
2. Otwórz szablon, w którym chcesz, aby licznik się pojawił. Na przykład szablon **Blog** (lista postów) lub sekcja listy produktów.
3. Kliknij **Add block** w sekcji, która renderuje każdy element.
4. W sekcji **Apps** wybierz **FastComments - Comment Count**.
5. Kliknij **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Nadpisuje tenant FastComments, z którego pobierany jest licznik. Pozostaw puste, aby użyć tenant skonfigurowanego automatycznie dla sklepu. | (puste) |
| Custom URL ID | Nadpisuje identyfikator strony, którego używa licznik. Użyj tego, gdy licznik znajduje się na innej stronie niż blok FastComments, który śledzi. | (wykrywane automatycznie) |

### How the count matches the comment thread

Blok Comment Count używa tej samej logiki automatycznego wykrywania co blok **FastComments**:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Jeśli ustawisz **Custom URL ID** w bloku **FastComments** na stronie, ustaw ten sam Custom URL ID w bloku Comment Count, aby wskazywały ten sam wątek.

### Tips

- Liczniki dla wszystkich elementów na stronie są pobierane w jednym żądaniu, więc dodanie bloku do każdego elementu w długiej liście nie generuje dodatkowych zapytań.
- Zalecane użycie to jeden blok Comment Count na artykuł lub produkt w zestawieniu; blok można dodać tyle razy, ile potrzebujesz.