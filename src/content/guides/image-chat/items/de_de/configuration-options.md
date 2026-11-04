### Übersicht

FastComments Image Chat erweitert das Standard-FastComments-Kommentierungs-Widget und erbt daher alle Konfigurationsoptionen des Basis-Widgets, während einige spezifische Optionen für Bildannotationen hinzukommen.

### Erforderliche Konfiguration

#### tenantId

Ihre FastComments Tenant ID ist erforderlich. Sie finden diese in Ihrem [FastComments-Dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat spezifische Optionen

#### urlId

Standardmäßig generiert Image Chat eine eindeutige Kennung für jede Konversation basierend auf der Seiten-URL, der Bildquelle und den X/Y-Koordinaten. Sie können dies mit einer benutzerdefinierten `urlId` überschreiben.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Dies ist nützlich, wenn sich Ihre URL-Struktur ändern könnte, Sie aber dieselben Konversationen behalten möchten, oder wenn Sie Annotationen über mehrere Seiten hinweg teilen möchten.

#### chatSquarePercentage

Steuert die Größe der anklickbaren Chat-Marker als Prozentsatz der Bildbreite. Der Standardwert ist 5 %, d. h. jeder Marker ist 5 % der Bildbreite.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Größere, besser sichtbare Marker
});
```

Kleinere Werte erzeugen unauffälligere Marker, die sich besser für detaillierte Bilder eignen. Größere Werte machen Marker leichter sichtbar und anklickbar bei vielbeschäftigten Bildern oder für Nutzer auf Mobilgeräten.

#### hasDarkBackground

Aktivieren Sie das Dark-Mode-Styling, wenn Ihre Seite einen dunklen Hintergrund hat.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Eine Callback-Funktion, die ausgelöst wird, sobald sich die Kommentaranzahl ändert. Dies ist nützlich, um UI-Elemente wie Badges oder Seitentitel zu aktualisieren.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Geerbte Konfigurationsoptionen

Da Image Chat das Standard-Kommentierungs-Widget erweitert, können Sie jede Konfigurationsoption des Basis-FastComments-Widgets verwenden. Hier sind einige häufig verwendete Optionen:

#### locale

Legt die Sprache für die Widget-UI fest. FastComments unterstützt Dutzende Sprachen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanisch
});
```

#### readonly

Macht alle Konversationen schreibgeschützt. Nutzer können vorhandene Marker und Diskussionen anzeigen, aber keine neuen erstellen oder antworten.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso und simpleSSO

Integrieren Sie Ihr Authentifizierungssystem über Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO-Konfiguration
    }
});
```

Details zu den Authentifizierungsoptionen finden Sie in der SSO-Dokumentation.

#### maxReplyDepth

Steuert, wie viele Ebenen tief Antworten gehen können. Standardmäßig setzt Image Chat dies auf 0, was bedeutet, dass alle Kommentare flach sind (keine verschachtelten Antworten). Sie können dies ändern, wenn Sie Thread-Konversationen wünschen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Bis zu 3 Verschachtelungsebenen erlauben
});
```

### Interne Konfiguration

Diese Optionen werden von Image Chat automatisch gesetzt und sollten nicht überschrieben werden:

Die `productId` wird für Image Chat automatisch auf `2` gesetzt. Die `floating-chat`-Erweiterung wird automatisch geladen, um die Funktionalität des Chat-Fensters bereitzustellen. Das Widget erkennt Mobilgeräte automatisch (Bildschirme unter 768px Breite) und passt die UI entsprechend mit Vollbild-Chat-Fenstern an.

### Flexibilität des Zielelements

Der erste Parameter von `FastCommentsImageChat` kann entweder direkt ein `<img>`-Element oder ein Containerelement mit einem Bild darin sein:

```javascript
// Direktes Bildelement
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container mit Bild darin
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Das Widget findet das Bild automatisch, wenn Sie ein Containerelement übergeben.

### Vollständiges Beispiel

Hier ist ein Beispiel, das mehrere Konfigurationsoptionen zusammen zeigt:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Ihre SSO-Konfiguration
    },
    maxReplyDepth: 1
});
```

Eine vollständige Liste aller verfügbaren Konfigurationsoptionen, die vom Basis-Widget geerbt werden, finden Sie in der Hauptdokumentation zur FastComments-Konfiguration.