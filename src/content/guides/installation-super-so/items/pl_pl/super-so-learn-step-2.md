W następnym kroku skopiuj poniższy gotowy kod widżetu.

Jeśli jesteś zalogowany na FastComments.com, poniższy fragment kodu będzie już zawierał informacje o Twoim koncie. Skopiujmy go:

[inline-code-attrs-start title = 'Kod Collab Chat FastComments dla Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Usuń istniejącą instancję
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Usuń istniejący pasek u góry, jeśli istnieje
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Utwórz nowy pasek u góry
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Zainicjalizuj FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Zaktualizuj bieżącą wartość pathname
            currentPathname = window.location.pathname;
        }

        // Początkowe ładowanie
        load();

        // Sprawdzaj co 500 ms, czy zaszły zmiany
        setInterval(() => {
            // Ponownie załaduj, jeśli pathname uległ zmianie
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponownie załaduj, jeśli widżet został usunięty
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponownie załaduj, jeśli kontener został opróżniony
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Teraz wklej w obszar `Body`:

<div class="screenshot white-bg">
    <div class="title">Wklejony kod</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Wklejony kod" />
</div>

If you see a "this is a demo message" after pasting the code:

- Upewnij się, że jesteś zalogowany na swoje konto na fastcomments.com.
- Upewnij się, że masz włączone ciasteczka stron trzecich.
- Odśwież tę stronę i skopiuj fragment kodu ponownie. Powinien mieć wypełnione pole `tenantId` identyfikatorem Twojego tenanta.