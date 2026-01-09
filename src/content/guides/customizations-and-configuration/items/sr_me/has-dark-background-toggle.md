[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Za sajtove koji dozvoljavaju prebacivanje tamnog režima nakon inicijalnog učitavanja stranice, ovo je malo složenije.

Prvo, sve aktuelne verzije biblioteke widgeta za komentare (React, Vue) imaju primjere prebacivanja tamnog režima u svojim odgovarajućim repozitorijumima.

Za VanillaJS widget, moraćemo da uradimo malo više posla. Prvo, FastCommentsUI vraća objekat sa funkcijama "destroy" i "update".

Možemo jednostavno pozvati funkciju update svaki put kada želimo da ažuriramo konfiguraciju widgeta za komentare, kako slijedi. Evo kompletnog funkcionalnog primjera prebacivanja
tamnog režima sa VanillaJS widgetom.

[inline-code-attrs-start title = 'Kompletan primjer prebacivanja tamnog režima'; inline-code-attrs-end]
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