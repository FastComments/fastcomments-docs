### Dunkelmodus-Unterstützung

### Dynamischer Dunkelmodus

Wenn der Dunkelmodus Ihrer Seite durch Hinzufügen der Klasse `.dark` zum body-Element gesteuert wird, respektiert die Collab Chat UI dies automatisch, ohne dass eine Neuinitialisierung erforderlich ist. Die Styles des Widgets sind so gestaltet, dass sie auf das Vorhandensein dieser Klasse reagieren.

[inline-code-attrs-start title = 'Beispiel für Dark Mode CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Ihr Dark-Mode-CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Benutzerdefinierte Gestaltung mit CSS

Sie können das Erscheinungsbild von Hervorhebungen, Chatfenstern und anderen Elementen mit CSS anpassen. Das Widget fügt spezifische Klassen hinzu, die Sie in Ihrem Stylesheet ansprechen können.

Text-Hervorhebungen verwenden das FastComments-Kommentarblasen-Styling-System, sodass alle Anpassungen, die Sie am Standard-Kommentar-Widget vorgenommen haben, sich auch auf Collab Chat auswirken.

### Anpassung der oberen Leiste

Die obere Leiste zeigt die Anzahl der Online-Nutzer und die Anzahl der Diskussionen an. Sie können ihre Position anpassen, indem Sie ein benutzerdefiniertes Element als `topBarTarget` angeben:

[inline-code-attrs-start title = 'Benutzerdefinierter Ort der oberen Leiste'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Oder deaktivieren Sie sie vollständig, indem Sie sie auf `null` setzen:

[inline-code-attrs-start title = 'Obere Leiste deaktivieren'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Verhalten auf Mobilgeräten

Auf Bildschirmen unter 768px Breite wechselt Collab Chat automatisch in ein für Mobilgeräte optimiertes Layout. Chatfenster erscheinen im Vollbildmodus anstatt neben dem Text zu schweben, und die Auswahlverzögerung wird entfernt, um eine unmittelbarere Interaktion zu ermöglichen.

Dieses Verhalten ist eingebaut und erfordert keine Konfiguration. Das Widget erkennt die Bildschirmgröße automatisch und passt sich entsprechend an.

### Darstellung der Chatfenster

Chatfenster sind auf Desktop 410px breit und haben einen 16px-Pfeil, der auf den hervorgehobenen Text zeigt. Die Fenster positionieren sich automatisch basierend auf dem verfügbaren Viewport-Platz und verwenden Positionierungsklassen wie `to-right`, `to-left`, `to-top` und `to-bottom`.

Sie können benutzerdefiniertes CSS hinzufügen, um Farben, Schriftarten, Abstände oder andere visuelle Eigenschaften dieser Fenster anzupassen. Die Chatfenster verwenden dieselbe Komponentenstruktur wie das Standard-FastComments-Widget, sodass sie alle von Ihnen vorgenommenen globalen Anpassungen erben.

### Lokalisierung

Collab Chat unterstützt dieselben Lokalisierungsoptionen wie das Standard-FastComments-Widget. Setzen Sie die Option `locale`, um die UI-Texte in verschiedenen Sprachen anzuzeigen:

[inline-code-attrs-start title = 'Locale festlegen'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanisch
});
[inline-code-end]

FastComments unterstützt dutzende Sprachen. Die Locale-Einstellung beeinflusst alle UI-Texte, einschließlich Aufforderungen, Schaltflächen und Platzhaltertexte.

### Vererbte Anpassungsoptionen

Da Collab Chat das Standard-Commenting-Widget erweitert, erbt es alle Anpassungsoptionen des Basis-Widgets. Dazu gehören benutzerdefinierte CSS-Klassen, benutzerdefinierte Übersetzungen, Avatar-Anpassungen, Datumsformatierung und vieles mehr.

Siehe die Haupt-FastComments-Anpassungsdokumentation für die vollständige Liste der verfügbaren Anpassungsoptionen.

### Arbeiten mit benutzerdefinierten Schriftarten

Wenn Ihre Seite benutzerdefinierte Schriftarten verwendet, übernimmt die Collab Chat UI diese Schriftarten aus dem CSS Ihrer Seite. Möglicherweise müssen Sie eine Widget-Anpassungsregel erstellen und in dieser Regel `@import` alle Schriftarten in das benutzerdefinierte CSS aufnehmen, wenn Sie
wollen, dass das schwebende Chatfenster dieselben Schriftarten verwendet.

---