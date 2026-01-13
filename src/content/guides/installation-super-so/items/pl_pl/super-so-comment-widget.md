## Dodanie widżetu komentarzy na żywo do artykułów Notion w Super.so

Oprócz Collab Chat, możesz dodać tradycyjny widżet komentarzy na dole swoich artykułów Notion. Pozwala to czytelnikom zostawiać komentarze i prowadzić dyskusje dotyczące całego artykułu.

### Kroki instalacji

Skopiuj poniższy kod i wklej go w sekcji `Body` ustawień witryny Super.so:

[inline-code-attrs-start title = 'Widżet komentarzy na żywo FastComments dla Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Usuń istniejącą instancję
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Utwórz nowy element docelowy
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Zainicjuj FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Zaktualizuj bieżącą ścieżkę
            currentPathname = window.location.pathname;
        }

        // Początkowe ładowanie
        load();

        // Sprawdzaj zmiany co 500 ms
        setInterval(() => {
            // Przeładuj, jeśli ścieżka się zmieniła
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Przeładuj, jeśli widżet został usunięty
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Przeładuj, jeśli kontener został opróżniony
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Ważne uwagi

- Widżet komentarzy pojawi się na dole Twoich artykułów Notion
- Każda strona otrzymuje własny, unikalny wątek komentarzy oparty na ścieżce URL
- Upewnij się, że zamienisz `"demo"` na rzeczywiste ID najemcy z Twojego konta FastComments
- Widżet automatycznie obsługuje dynamiczne ładowanie stron Super.so