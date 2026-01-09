[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Voor sites die het wisselen van de donkere modus na het initiële laden van de pagina toestaan, is dit iets ingewikkelder.

Allereerst bevatten alle huidige versies van de Comment-widgetbibliotheek (React, Vue) voorbeelden van het wisselen van de donkere modus in hun respectieve repositories.

Voor de VanillaJS-widget moeten we wat meer werk doen. Ten eerste retourneert de FastCommentsUI een object met de functies "destroy" en "update".

We kunnen eenvoudig de update-functie aanroepen telkens wanneer we de configuratie van de commentaarwidget willen bijwerken, zoals hieronder. Hier is een volledig werkend voorbeeld van het wisselen van de donkere modus met de VanillaJS-widget.

[inline-code-attrs-start title = 'Volledig voorbeeld: Donkere modus wisselen'; inline-code-attrs-end]
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

---