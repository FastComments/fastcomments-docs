Für Schritt 2 müssen wir unseren Code-Snippet kopieren. Überprüfen Sie, dass Zeile 50 nicht "demo" sagt – Sie sollten sicherstellen, dass hier Ihre Tenant-ID steht. Sie sollte für Sie ausgefüllt sein.

Kopieren wir nun unseren ThriveCart-Learn-spezifischen FastComments-Code-Snippet.

Er ist ziemlich groß, weil die Integration mit ThriveCart viele Funktionen hat. Klicken Sie einfach auf die Schaltfläche "Copy" oben rechts im Code-Snippet:

[inline-code-attrs-start title = 'ThriveCart Learn+ Kommentar-Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // Klasse ist in der Vorschau anders.
            }
            // allgemeiner E-Mail-Eingabefeld-Selektor für den Fall, dass ThriveCart die ID ändert.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // Vorschau erlauben - keine E-Mail verfügbar.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // Wartezeit nach 5 Versuchen erhöhen, falls langsames Internet.
            }
            if (profileLink) {
                // Rohes "img"-Query verwenden, falls ThriveCart die Bild-Klassen-Selector ändert.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // innerText verwenden, falls ThriveCart die Anzeige des Profilnamens ändert.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // Wartezeit nach 5 Versuchen erhöhen, falls langsames Internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // Manchmal verwendet TC mehrere Links auf derselben Seite, daher doppelte entfernen.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // Marketing-Parameter und Domainnamen abschneiden
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // Marketing-Parameter und Domainnamen abschneiden
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // Standard auf aktuelle, damit es zumindest manchmal funktioniert
            }
        }

    })();
</script>
[inline-code-end]

Fügen Sie es nun in das Code-Feld links im ThriveCart-Editor ein. Es sollte so aussehen:

<div class="screenshot white-bg">
    <div class="title">Code hinzugefügt</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code hinzugefügt" />
</div>

Das war's! Jetzt müssen wir es nur noch veröffentlichen:

<div class="screenshot white-bg">
    <div class="title">Veröffentlichen</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Veröffentlichen" />
</div>

Das war's! Sie sollten jetzt das Kommentarfeld in Ihrem Kurs sehen, wenn Sie die Vorschau aufrufen, und echte Benutzer können Kommentare hinterlassen **ohne sich anzumelden oder ihren Benutzernamen/Ihre E-Mail ein zweites Mal anzugeben**.

### Testhinweis!

Wenn Sie anonymes Kommentieren deaktiviert haben, was standardmäßig der Fall ist, können Sie im `Preview`-Modus als der Benutzer `John Smith` keine Kommentare hinterlassen. Sie erhalten einen Authentifizierungsfehler, da der Standardbenutzer `John Smith` keine E-Mail hat. Wenn Sie testen möchten, empfehlen wir, einen Gutscheincode zu verwenden und die Seite wie ein echter Benutzer zu durchlaufen.