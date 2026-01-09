Następnie dodamy kod widgetu FastComments do Twojej witryny. Ten kod wyszuka wszystkie formularze o tytule `FastComments Goes Here` i
zastąpi je FastComments.

Przejdźmy więc do `Settings` w lewym dolnym rogu edytora strony:

<div class="screenshot white-bg">
    <div class="title">Otwórz ustawienia</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Otwórz ustawienia" />
</div>

Otwórz sekcję `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Otwórz Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Otwórz Custom Head Code" />
</div>

Dla Ionos potrzebujemy **specjalnej wersji** kodu widgetu FastComments. Fragmenty kodu z **innych samouczków nie będą działać.**

Teraz skopiuj następujący kod:

[inline-code-attrs-start title = 'Fragment FastComments dla Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // pobieramy element, który nie ma pełnej szerokości
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

...i wklej go, jak pokazano:

<div class="screenshot white-bg">
    <div class="title">Wklej i zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Wklej i zapisz" />
</div>

---