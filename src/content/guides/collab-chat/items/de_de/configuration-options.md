### Übersicht

FastComments Collab Chat erweitert das Standard-FastComments-Kommentarmodul und übernimmt daher alle Konfigurationsoptionen des Basismoduls, während es einige spezifische Optionen für Textannotationen hinzufügt.

### Erforderliche Konfiguration

#### tenantId

Ihre FastComments Tenant-ID ist erforderlich. Sie finden diese in Ihrem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat-spezifische Optionen

#### urlId

Standardmäßig generiert Collab Chat eine eindeutige Kennung für jede Unterhaltung basierend auf der Seiten-URL, dem DOM-Pfad zum Element und dem ausgewählten Textbereich. Sie können dies mit einer benutzerdefinierten `urlId` überschreiben.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Dies ist nützlich, wenn sich Ihre URL-Struktur ändern könnte, Sie aber die gleichen Unterhaltungen beibehalten möchten, oder wenn Sie Annotationen über mehrere Seiten hinweg teilen möchten.

#### topBarTarget

Steuert die Anzeige der oberen Leiste, die die Benutzeranzahl und die Diskussionsanzahl anzeigt. Setzen Sie auf `null`, um die obere Leiste vollständig zu deaktivieren, oder geben Sie ein DOM-Element an, um sie an einem bestimmten Ort zu rendern.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Top-Leiste deaktivieren
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Top-Leiste an benutzerdefiniertem Ort rendern
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Aktivieren Sie das Dark-Mode-Styling, wenn Ihre Seite einen dunklen Hintergrund hat. Diese Erkennung erfolgt automatisch, kann aber ggf. überschrieben werden.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Eine Callback-Funktion, die ausgelöst wird, sobald sich die Kommentaranzahl ändert. Dies ist nützlich, um UI-Elemente wie Badges oder Seitentitel zu aktualisieren.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Geerbte Konfigurationsoptionen

Da Collab Chat das Standard-Kommentarmodul erweitert, können Sie jede Konfigurationsoption des Basismoduls von FastComments verwenden. Hier sind einige häufig verwendete Optionen:

#### locale

Legen Sie die Sprache für die Widget-Benutzeroberfläche fest. FastComments unterstützt Dutzende von Sprachen.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanisch
});
[inline-code-end]

#### readonly

Machen Sie alle Unterhaltungen schreibgeschützt. Benutzer können vorhandene Annotationen ansehen, aber keine neuen erstellen oder antworten.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso und simpleSSO

Integrieren Sie Ihr Authentifizierungssystem mit Single Sign-On.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO-Konfiguration
    }
});
[inline-code-end]

Siehe die SSO-Dokumentation für vollständige Details zu den Authentifizierungsoptionen.

#### maxReplyDepth

Steuert, wie tief Antworten verschachtelt werden können. Standardmäßig setzt Collab Chat dies auf 0, was bedeutet, dass alle Kommentare flach sind (keine verschachtelten Antworten). Sie können dies ändern, wenn Sie verschachtelte Konversationen wünschen.

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Erlaube 3 Ebenen der Verschachtelung
});
[inline-code-end]

### Interne Konfiguration

Diese Optionen werden automatisch von Collab Chat gesetzt und sollten nicht überschrieben werden:

Die Eigenschaft `productId` wird automatisch auf `3` für Collab Chat gesetzt. Die Erweiterung `floating-chat` wird automatisch geladen, um die Chatfenster-Funktionalität bereitzustellen. Das Widget erkennt automatisch mobile Geräte (Bildschirme unter 768px Breite) und passt die Benutzeroberfläche entsprechend an.

### Vollständiges Beispiel

Hier ein Beispiel, das mehrere Konfigurationsoptionen zusammen zeigt:

[inline-code-attrs-start title = "Konfigurationsbeispiel"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Ihre SSO-Konfiguration
    },
    maxReplyDepth: 1
});
[inline-code-end]

Für eine vollständige Liste aller verfügbaren Konfigurationsoptionen, die vom Basismodul geerbt werden, siehe die Hauptdokumentation zur FastComments-Konfiguration.