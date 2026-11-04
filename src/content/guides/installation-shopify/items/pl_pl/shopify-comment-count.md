Blok **FastComments - Comment Count** wyświetla mały licznik komentarzy dla pojedynczej strony. Używaj go na listach wpisów na blogu, kartach produktów ani w żadnym szablonie, który prowadzi do strony z komentarzami, aby odwiedzający mogli zobaczyć, jak aktywny jest każdy wątek, zanim klikną.

### Dodanie bloku

1. Otwórz edytor motywu Shopify.
2. Otwórz szablon, w którym ma się pojawić licznik. Na przykład szablon **Blog** (lista wpisów) lub sekcja z listą produktów.
3. Kliknij **Add block** w sekcji, która renderuje każdy element.
4. W sekcji **Apps** wybierz **FastComments - Comment Count**.
5. Kliknij **Save**.

### Ustawienia

| Ustawienie | Co robi | Wartość domyślna |
|---|---|---|
| Tenant ID (opcjonalne) | Nadpisuje, z którego tenanta FastComments licznik pobiera dane. Pozostaw puste, aby użyć automatycznie skonfigurowanego tenanta sklepu. | (puste) |
| Custom URL ID | Nadpisuje identyfikator strony, którego szuka licznik. Użyj tego, gdy licznik znajduje się na innej stronie niż blok FastComments, który śledzi. | (wykrywane automatycznie) |

### Jak licznik dopasowuje wątek komentarzy

Blok Comment Count używa tej samej logiki automatycznego wykrywania co blok **FastComments**:

- Szablon wpisu na blogu: `shopify-article-{article.id}`
- Szablon produktu: `shopify-product-{product.id}`
- Inne szablony: ścieżka żądania

Jeśli ustawiłeś **Custom URL ID** w bloku **FastComments** na stronie, ustaw ten sam Custom URL ID w bloku Comment Count, aby wskazywały ten sam wątek.

### Wskazówki

- Liczniki dla wszystkich elementów na stronie są pobierane w jednym żądaniu, więc dodanie bloku do każdego elementu w długiej liście nie generuje dodatkowych zapytań.
- Jeden blok Comment Count na artykuł lub produkt na liście to oczekiwane użycie; blok można dodać tyle razy, ile potrzebujesz.