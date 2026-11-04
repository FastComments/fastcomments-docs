[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

For sider, der tillader at skifte dark mode efter den indledende sideindlæsning, er dette lidt mere kompliceret.

Først indeholder alle aktuelle versioner af Comment widget-biblioteket (React, Vue) eksempler på at skifte dark mode i deres respektive repositories.

For VanillaJS-widgetten skal vi lave lidt mere arbejde. Først returnerer FastCommentsUI et objekt med funktionerne "destroy" og "update".

Vi kan simpelthen kalde update-funktionen, hver gang vi vil opdatere comment widget-konfigurationen, som følger. Her er et komplet fungerende eksempel på at skifte
dark mode med VanillaJS-widgetten.

[inline-code-attrs-start title = 'Komplet eksempel på skift af Dark Mode'; inline-code-attrs-end]
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