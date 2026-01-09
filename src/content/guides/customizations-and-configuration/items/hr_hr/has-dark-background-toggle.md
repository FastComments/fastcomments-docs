[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Za web-lokacije koje omogućuju prebacivanje tamnog načina nakon početnog učitavanja stranice, ovo je malo složenije.

Prvo, sve trenutne verzije biblioteke Comment widget (React, Vue) imaju primjere za prebacivanje tamnog načina u svojim odgovarajućim spremištima.

Za VanillaJS widget morat ćemo napraviti još posla. Prvo, FastCommentsUI vraća objekt s funkcijama "destroy" i "update".

Možemo jednostavno pozvati funkciju update svaki put kad želimo ažurirati konfiguraciju widgeta komentara, kako slijedi. Evo potpunog funkcionalnog primjera prebacivanja
tamnog načina s VanillaJS widgetom.

[inline-code-attrs-start title = 'Potpuni primjer prebacivanja tamnog načina'; inline-code-attrs-end]
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