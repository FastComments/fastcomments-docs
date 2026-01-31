Teraz możemy skopiować poniższy fragment kodu. Użyj przycisku kopiowania, który pojawia się w prawym górnym rogu fragmentu.

W kodzie można skonfigurować kilka rzeczy, zobacz linie 4–7.

[inline-code-attrs-start title = 'Kod pojedynczej strony Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // Twój identyfikator konta
    }];
</script>
[inline-code-end]

Powinno to wyglądać tak:

<div class="screenshot white-bg">
    <div class="title">Wklej i zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Wklej i zapisz" />
</div>

Teraz kliknij Zapisz w prawym górnym rogu.

Zauważ, że opcja `Preview in Safe Mode` nie będzie działać, ale widżet pojawi się po odwiedzeniu Twojej witryny.

Jeśli napotkasz problemy, upewnij się, że na dole nie jest napisane `"tenantId": "demo"`. Powinno pokazywać Twój tenant id, jeśli jesteś zalogowany. Jeśli nie, skontaktuj się z pomocą techniczną.