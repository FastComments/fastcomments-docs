Next ćemo dodati FastComments widget kod na vaš sajt. Ovaj kod će pretražiti sve forme sa naslovom `FastComments Goes Here` i
zamijeniti ih sa FastComments.

Dakle, idemo u `Settings` u donjem lijevom uglu uređivača sajta:

<div class="screenshot white-bg">
    <div class="title">Otvorite Settings</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Otvorite Settings" />
</div>

Otvorite sekciju `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Otvorite Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Otvorite Custom Head Code" />
</div>

Za Ionos trebamo **posebnu verziju** FastComments widget koda. Kodovi iz **ostalih tutorijala neće raditi.**

Sada kopirajte sljedeći kod:

[inline-code-attrs-start title = 'Ionos FastComments isječak'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // dohvatimo element koji nije pune širine
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...i zalijepite ga kako je prikazano:

<div class="screenshot white-bg">
    <div class="title">Zalijepite i sačuvajte</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Zalijepite i sačuvajte" />
</div>

---