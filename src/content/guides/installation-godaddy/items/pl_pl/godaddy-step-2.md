Teraz, gdy dodałeś niestandardowy blok HTML, możemy dodać kod widżetu FastComments.

**Użyj poniższego kodu dla Godaddy — nie kodu z innych samouczków. Ten kod jest specyficzny dla Godaddy.**

Skopiuj poniższy kod:

[inline-code-attrs-start title = 'Fragment kodu komentarzy Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Ten fragment kodu jest zaprojektowany tak, aby był kompatybilny z Godaddy i będzie wyświetlany tylko w Twoich wpisach na blogu — nie na stronie głównej.

Teraz wklej kod w obszar `Custom Code` wspomniany w `Step One`.

<div class="screenshot white-bg">
    <div class="title">Skopiuj i wklej kod</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Skopiuj i wklej kod" />
</div>

Kliknij Gotowe w prawym górnym rogu:

<div class="screenshot white-bg">
    <div class="title">Kliknij Gotowe</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Kliknij Gotowe" />
</div>

To wszystko dla kroku drugiego!