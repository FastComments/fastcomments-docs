[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Für Websites, die das Umschalten des Dunkelmodus nach dem initialen Laden der Seite erlauben, ist dies etwas komplizierter.

Zunächst enthalten alle aktuellen Versionen der Comment-Widget-Bibliothek (React, Vue) Beispiele zum Umschalten des Dunkelmodus in ihren jeweiligen Repositories.

Für das VanillaJS-Widget müssen wir etwas mehr Arbeit leisten. Die FastCommentsUI gibt zunächst ein Objekt mit den Funktionen "destroy" und "update" zurück.

Wir können einfach die Funktion update aufrufen, jedes Mal wenn wir die Konfiguration des Kommentar-Widgets aktualisieren möchten, wie folgt. Hier ist ein voll funktionsfähiges Beispiel zum Umschalten des Dunkelmodus mit dem VanillaJS-Widget.

[inline-code-attrs-start title = 'Vollständiges Beispiel: Umschalten des Dunkelmodus'; inline-code-attrs-end]
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