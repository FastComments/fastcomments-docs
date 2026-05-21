Blok **FastComments - Reviews Summary** pokazuje zebrane oceny w gwiazdkach i rozkład recenzji dla strony. Połącz go z blokiem **FastComments** na szablonach produktów, aby uzyskać standardowy układ recenzji: podsumowanie u góry, formularz recenzji i recenzje poniżej.

### Wymóg wstępny: skonfiguruj Ratings & Reviews

Blok Reviews Summary wyświetla pytania ocen, które skonfigurowałeś dla swojego sklepu. Najpierw je ustaw:

1. Otwórz aplikację FastComments w panelu administracyjnym Shopify.
2. Kliknij kafelek **Ratings & Reviews Helper** (lub otwórz bezpośrednio [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)).
3. Dodaj pytania, na które chcesz, aby odpowiadał każdy recenzent (ogólna ocena w gwiazdkach, "jak pasuje", itp.).

Bez skonfigurowanych pytań blok podsumowania nie ma czego agregować.

### Dodaj blok

1. Otwórz edytor motywu Shopify.
2. Otwórz szablon **Product** (lub szablon strony, na którym chcesz mieć podsumowanie).
3. Kliknij **Add block** w górnej części sekcji strony, nad miejscem, gdzie będzie blok **FastComments**.
4. W sekcji **Apps** wybierz **FastComments - Reviews Summary**.
5. Dodaj blok **FastComments** niżej na tej samej stronie, jeśli jeszcze go nie masz, aby odwiedzający mogli zostawiać recenzje.
6. Kliknij **Save**.

### Ustawienia

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Nadpisuje, z którego tenanta FastComments ma odczytywać podsumowanie. Pozostaw puste, aby użyć automatycznie skonfigurowanego tenant dla sklepu. | (blank) |
| Custom URL ID | Nadpisuje identyfikator strony, według którego agregowane jest podsumowanie. Użyj tego, gdy podsumowanie znajduje się na innej stronie niż blok FastComments, który odzwierciedla. | (auto-detected) |

### Jak podsumowanie pasuje do recenzji

Blok Reviews Summary używa tej samej logiki automatycznego wykrywania co blok **FastComments**:

- Product template: `shopify-product-{product.id}`
- Blog post template: `shopify-article-{article.id}`
- Other templates: the request path

Dla normalnej strony produktu podsumowanie i wątek komentarzy automatycznie dzielą ten sam URL ID, bez potrzeby dodatkowej konfiguracji.

### Wskazówki

- Podsumowanie jest tylko do odczytu. Aby zbierać recenzje, potrzebujesz bloku **FastComments** na tej samej stronie.
- Jeśli zmienisz pytania ocen w Ratings & Reviews Helper po zebraniu recenzji, podsumowanie przeliczy się względem nowego zestawu pytań.