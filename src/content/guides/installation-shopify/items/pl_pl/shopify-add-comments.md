Blok **FastComments** jest głównym widżetem komentowania. Dodaj go do szablonów postów na blogu, szablonów produktów lub dowolnej innej strony, na której chcesz wątek dyskusji lub czat na żywo.

### Dodaj blok

1. Otwórz edytor motywu Shopify (**Online Store > Themes > Customize**).
2. Wybierz szablon, na którym chcesz mieć komentarze: **Blog post**, **Product** lub dowolny inny szablon strony lub sekcji.
3. W sekcji, w której chcesz, aby pojawiły się komentarze, kliknij **Add block**.
4. W sekcji **Apps** wybierz **FastComments**.
5. Kliknij **Save**.

Blok pojawia się natychmiast. Nie trzeba wpisywać Tenant ID; tenant Twojego sklepu zostanie skonfigurowany automatycznie po zainstalowaniu aplikacji.

### Ustawienia

| Setting | Co robi | Domyślnie |
|---|---|---|
| Tenant ID (optional) | Nadpisuje tenant FastComments, względem którego blok jest renderowany. Pozostaw puste, aby użyć tenanta skonfigurowanego automatycznie dla sklepu. Znajdź ręczny tenant ID na fastcomments.com/auth/my-account/api-secret. | (puste) |
| SSO | Automatycznie loguje odwiedzającego jako jego konto klienta Shopify przed komentowaniem. Zobacz [Automatyczne logowanie klientów Shopify](/guide-installation-shopify.html#shopify-sso). | Włączone |
| Commenting Style | Wybierz **Threaded** dla zagnieżdżonych odpowiedzi i głosów, lub **Streaming** dla strumienia czatu w czasie rzeczywistym. | Threaded |
| Custom URL ID | Nadpisuje automatycznie wykryty identyfikator strony. Użyj tego, gdy chcesz, aby dwa adresy URL współdzieliły jeden wątek komentarzy. | (wykrywane automatycznie) |

### Jak wybierany jest identyfikator strony

Każdy wątek komentarzy jest kluczowany przez identyfikator URL. Blok wybiera go automatycznie:

- **Blog post template:** `shopify-article-{article.id}`, który jest stabilny przy zmianach slug i tytułu.
- **Product template:** `shopify-product-{product.id}`, który jest stabilny przy zmianach slug i tytułu.
- **Other templates:** ścieżka żądania.

Jeśli ustawisz **Custom URL ID**, używana będzie zamiast tego ta wartość. Użyj tego samego Custom URL ID w kilku blokach (na przykład w zlokalizowanej wersji strony produktu), aby współdzielić jeden wątek komentarzy.

### Threaded vs Streaming

**Threaded** jest domyślne. Odwiedzający odpowiadają sobie nawzajem, głosują, a narzędzia do moderacji działają jak oczekiwano. Najlepsze do postów na blogu i recenzji produktów.

**Streaming** usuwa zagnieżdżanie i pokazuje nowe komentarze w czasie rzeczywistym w momencie ich dodania, jak strumień czatu. Najlepsze do premier produktów, wydarzeń na żywo i stron społecznościowych.

### Kilka bloków na tej samej stronie

Blok można dodać więcej niż raz do tego samego szablonu. Na przykład podsumowanie recenzji na górze strony produktu i blok FastComments na dole. Bloki współdzielą identyfikator URL, dzięki czemu podsumowanie odzwierciedla komentarze poniżej.

### Wskazówki

- Blok ukrywa się w podglądzie edytora motywu z żółtym powiadomieniem, jeśli nie może znaleźć tenant. Jeśli pojawi się to w Twoim działającym sklepie, ponownie zainstaluj aplikację FastComments.
- Na stronie produktu blok FastComments pełni jednocześnie rolę widżetu recenzji produktu. Połącz go z **FastComments - Reviews Summary**, aby uzyskać podsumowanie ocen gwiazdkowych na górze strony.