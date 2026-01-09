[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Dla stron, które umożliwiają przełączanie trybu ciemnego po początkowym załadowaniu strony, jest to trochę bardziej skomplikowane.

Po pierwsze, wszystkie obecne wersje biblioteki widżetu komentarzy (React, Vue) zawierają przykłady przełączania trybu ciemnego w swoich repozytoriach.

Dla widżetu VanillaJS będziemy musieli wykonać dodatkową pracę. Funkcja FastCommentsUI zwraca obiekt z funkcjami "destroy" i "update".

Możemy po prostu wywołać funkcję update za każdym razem, gdy chcemy zaktualizować konfigurację widżetu komentarzy, w następujący sposób. Oto kompletny działający przykład przełączania
trybu ciemnego przy użyciu widżetu VanillaJS.

[inline-code-attrs-start title = 'Kompletny przykład przełączania trybu ciemnego'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]