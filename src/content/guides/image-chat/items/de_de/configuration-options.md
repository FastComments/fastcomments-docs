### Überblick

FastComments Image Chat erweitert das standardmäßige FastComments-Kommentarmodul, sodass es alle Konfigurationsoptionen des Basis-Widgets übernimmt und zusätzlich einige speziell für Bildannotationen hinzufügt.

### Erforderliche Konfiguration

#### tenantId

Ihre FastComments Tenant ID wird benötigt. Sie finden diese in Ihrem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat-spezifische Optionen

#### urlId

Standardmäßig erzeugt Image Chat für jede Konversation eine eindeutige Kennung basierend auf der Seiten-URL, der Bildquelle und den X/Y-Koordinaten. Sie können dies mit einer benutzerdefinierten `urlId` überschreiben.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Das ist nützlich, wenn sich Ihre URL-Struktur ändern kann, Sie aber die gleichen Konversationen behalten möchten, oder wenn Sie Anmerkungen über mehrere Seiten hinweg teilen möchten.

#### chatSquarePercentage

Steuert die Größe der anklickbaren Chat-Markierungen als Prozentsatz der Bildbreite. Standardmäßig sind es 5 %, das heißt jede Markierung entspricht 5 % der Bildbreite.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Größere, besser sichtbare Marker
});
```

Kleinere Werte erzeugen weniger aufdringliche Markierungen, die sich besser für detailreiche Bilder eignen. Größere Werte machen die Markierungen leichter sichtbar und anklickbar bei überladenen Bildern oder für Nutzer mobiler Geräte.

#### hasDarkBackground

Aktivieren Sie das Dark-Mode-Styling, wenn Ihre Seite einen dunklen Hintergrund hat.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Eine Callback-Funktion, die ausgelöst wird, sobald sich die Kommentaranzahl ändert. Nützlich, um UI-Elemente wie Badges oder Seitentitel zu aktualisieren.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Vererbte Konfigurationsoptionen

Da Image Chat das Standard-Commenting-Widget erweitert, können Sie jede Konfigurationsoption des Basis-FastComments-Widgets verwenden. Hier sind einige häufig verwendete Optionen:

#### locale

Legt die Sprache für die Widget-Oberfläche fest. FastComments unterstützt Dutzende von Sprachen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanisch
});
```

#### readonly

Machen Sie alle Konversationen schreibgeschützt. Nutzer können vorhandene Markierungen und Diskussionen ansehen, aber keine neuen erstellen oder antworten.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrieren Sie Ihr Authentifizierungssystem mit Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO-Konfiguration
    }
});
```

Siehe die SSO-Dokumentation für vollständige Details zu den Authentifizierungsoptionen.

#### maxReplyDepth

Steuern Sie, wie tief Antworten verschachtelt sein dürfen. Standardmäßig setzt Image Chat dies auf 0, d.h. alle Kommentare sind flach (keine verschachtelten Antworten). Sie können dies ändern, wenn Sie verschachtelte Konversationen wünschen.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Erlaube 3 Verschachtelungsebenen
});
```

### Interne Konfiguration

Diese Optionen werden automatisch von Image Chat gesetzt und sollten nicht überschrieben werden:

Der `productId` wird für Image Chat automatisch auf `2` gesetzt. Die `floating-chat` Erweiterung wird automatisch geladen, um die Chatfenster-Funktionalität bereitzustellen. Das Widget erkennt automatisch mobile Geräte (Bildschirme mit weniger als 768px Breite) und passt die UI entsprechend mit Vollbild-Chatfenstern an.

### Flexibilität des Ziel-Elements

Der erste Parameter von `FastCommentsImageChat` kann entweder ein `<img>`-Element direkt oder ein Containerelement mit einem darin befindlichen Bild sein:

```javascript
// Direktes <img>-Element
FastCommentsImageChat(document.getElementById('my-image'), config);

// Container mit Bild darin
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Das Widget findet das Bild automatisch, wenn Sie ein Containerelement übergeben.

### Vollständiges Beispiel

Hier ein Beispiel, das mehrere Konfigurationsoptionen zusammen zeigt:

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

Für eine vollständige Liste aller verfügbaren Konfigurationsoptionen, die vom Basis-Widget übernommen werden, siehe die Hauptdokumentation zur FastComments-Konfiguration.