#### Wie Kommentare in Ihren Kursen angezeigt werden

Sobald die LTI-Integration aktiviert und die Externe App installiert ist, funktioniert FastComments automatisch basierend auf den von Ihnen konfigurierten Platzierungen:

#### Aufgabenansicht

Wenn die Platzierung **Aufgabenansicht** aktiviert ist, erscheinen Kommentare automatisch unter jeder Aufgabe im Kurs. Studierende und Lehrende sehen einen verschachtelten Kommentarbereich, wenn sie eine Aufgabe anzeigen — keine zusätzliche Einrichtung pro Aufgabe erforderlich.

Jede Aufgabe erhält ihren eigenen separaten Kommentarthread.

#### Schaltfläche des Rich Content Editors

Wenn die Platzierung **Editor-Schaltfläche** aktiviert ist, können Lehrende FastComments in jeden Inhalt einbetten, der den Rich Content Editor verwendet:

1. Bearbeiten Sie eine **Seite**, ein **Quiz** oder eine **Ankündigung**.
2. Klicken Sie in der Symbolleiste des Rich Content Editors auf die **FastComments**-Schaltfläche.
3. FastComments wird automatisch in den Inhalt eingebettet.
4. Speichern Sie die Seite.

Wenn Studierende die Seite aufrufen, lädt das eingebettete FastComments-Widget mit einem für diese Seite einzigartigen Kommentarthread.

#### Automatisches SSO

In beiden Platzierungen werden Studierende automatisch über ihr Canvas-Konto angemeldet. Namen, E‑Mails und Avatare werden über den LTI-Launch synchronisiert; ein separater Login ist nicht erforderlich.

#### Öffentlichen Zugriff einschränken (empfohlen)

Standardmäßig sind FastComments-Kommentardaten öffentlich lesbar. Jeder, der die URL eines Threads oder dessen API-Endpunkt errät, kann die Kommentare sehen, sogar außerhalb von Canvas. Für Kursdiskussionen möchten Sie die Ansicht sehr wahrscheinlich nur auf eingeschriebene Studierende beschränken.

Öffnen Sie Ihre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Widget-Anpassungsseite</a> und erstellen Sie eine Regel mit **SSO zum Anzeigen von Kommentaren erforderlich** aktiviert, und setzen Sie dann das Sicherheitsniveau auf **Sicheres SSO**, damit Threads nur über den signierten LTI-Launch geladen werden können.

Siehe [Schützen von Kommentar-Threads mit Single Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) für die vollständige Anleitung, einschließlich wie Sie die Regel auf eine einzelne Domain oder Seite eingrenzen.