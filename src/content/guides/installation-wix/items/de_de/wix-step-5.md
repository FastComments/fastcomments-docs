Als Nächstes richten wir alles so ein, dass sich der Kommentar-Thread basierend auf der aktuellen Seite ändert, sodass Benutzer über die aktuell angezeigten Inhalte diskutieren können.

Ohne die folgenden Schritte haben Sie nur einen einzigen globalen Kommentar-Thread für Ihre gesamte Website – was nicht sehr nützlich ist.

#### Entwicklermodus

Um diese Funktion hinzuzufügen, müssen wir in das, was Wix `Dev Mode` nennt, wechseln.

Klicken Sie auf die Option `Dev Mode` oben auf dem Bildschirm.

<div class="screenshot white-bg">
    <div class="title">Dev Mode aktivieren</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode aktivieren" />
</div>

#### Element-ID festlegen

Wir werden benutzerdefinierten Code hinzufügen, um dies zu erreichen, aber zuerst müssen wir dem neuen Embed-Element eine ID geben, auf die wir verweisen können.

Nennen wir es `fastcomments`.

Klicken Sie auf das neue Embed-Element, das wir hinzugefügt haben. Im Entwicklermodus sollten Sie unten rechts ein ID-Feld sehen mit einem Wert wie `html1`:

<div class="screenshot white-bg">
    <div class="title">Das ID-Feld</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Das ID-Feld" />
</div>

Ändern Sie dies zu `fastcomments` und drücken Sie Enter:

<div class="screenshot white-bg">
    <div class="title">ID setzen</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID setzen" />
</div>

Jetzt können wir unseren benutzerdefinierten Code hinzufügen, der dem Kommentarbereich mitteilt, welche Seite wir gerade ansehen.

Unten auf dem Bildschirm sollten Sie einen Code-Editor wie diesen sehen:

<div class="screenshot white-bg">
    <div class="title">Editor öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Editor öffnen" />
</div>

Kopieren Sie den folgenden Code und fügen Sie ihn dort ein:

[inline-code-attrs-start title = 'Wix-Kommentare Navigations-Snippet'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Fügen Sie den Navigationscode hinzu</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Navigationscode hinzufügen" />
</div>

---