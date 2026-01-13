[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Za spletna mesta, ki dovolijo preklapljanje temnega načina po začetnem nalaganju strani, je to nekoliko bolj zapleteno.

Najprej imajo vse trenutne različice knjižnice pripomočka Comment (React, Vue) primere preklapljanja temnega načina v svojih ustreznih repozitorijih.

Za pripomoček VanillaJS bo potrebno narediti še nekaj dodatnega dela. Najprej FastCommentsUI vrne objekt s funkcijama "destroy" in "update".

Preprosto lahko vsakokrat pokličemo funkcijo update, kadar želimo posodobiti konfiguracijo pripomočka za komentarje, kot sledi. Tukaj je popoln delujoč primer preklapljanja
temnega načina z uporabo pripomočka VanillaJS.

[inline-code-attrs-start title = 'Popoln primer preklopa temnega načina'; inline-code-attrs-end]
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