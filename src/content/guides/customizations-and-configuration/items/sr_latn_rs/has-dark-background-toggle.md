[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Za sajtove koji omogućavaju prebacivanje tamnog režima nakon početnog učitavanja stranice, ovo je malo složenije.

Prvo, sve aktuelne verzije biblioteke Comment widget (React, Vue) imaju primere prebacivanja tamnog režima u svojim odgovarajućim repozitorijumima.

Za VanillaJS widget, moraćemo da uradimo još nešto. Prvo, FastCommentsUI vraća objekat sa funkcijama "destroy" i "update".

Možemo jednostavno pozvati funkciju update svaki put kada želimo da ažuriramo konfiguraciju komentarskog widgeta, na sledeći način. Ovde je kompletan funkcioni primer prebacivanja
tamnog režima sa VanillaJS widgetom.

[inline-code-attrs-start title = 'Kompletan primer prebacivanja tamnog režima'; inline-code-attrs-end]
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