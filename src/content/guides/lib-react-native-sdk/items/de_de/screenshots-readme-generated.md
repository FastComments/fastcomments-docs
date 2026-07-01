Live-Threaded-Kommentare mit Avataren, verschachtelten Antworten, Abstimmungen und dem integrierten Rich‑Text‑Composer, plus einem Dark‑Theme und einer Live‑Chat‑Voreinstellung (hier dargestellt über `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich-Text-Editor

Diese Bibliothek verwendet [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) für Rich‑Text‑Bearbeitung, die ein leistungsstarkes WYSIWYG‑Bearbeitungserlebnis bietet. Derselbe Editor treibt iOS, Android und das Web (via `react-native-web`) an, sodass sich der Composer auf jeder Plattform mit einer einzigen Implementierung konsistent verhält.

`react-native-enriched` erfordert die neue React‑Native‑Architektur (Fabric) auf nativen Plattformen (Standard seit RN 0.76, optional ab RN 0.72‑0.75) und einen Bundler, der die `exports`‑Bedingungen des Pakets auflöst. Dieses SDK wurde entwickelt und getestet gegen RN 0.81 / React 19. Der gleiche Editor läuft ebenfalls im Web über `react-native-web`; das Web‑Build des enriched‑Editors ist nach wie vor als experimentell gekennzeichnet.

### Widgets

- `FastCommentsLiveCommenting` – Threaded‑Kommentare mit Abstimmungen, Antworten, Pagination, Erwähnungen, Benachrichtigungen und Live‑Updates.
- `FastCommentsLiveChat` – eine Chat‑Voreinstellung über dieselbe Engine: chronologische Nachrichten, wobei neue unten erscheinen, der Composer unter der Liste, ein Live‑Header‑Band (Verbindungs‑Punkt + Benutzerzahl), unbegrenzte Historie, die beim Hochscrollen geladen wird, automatisches Scrollen zu neuen Nachrichten, keine Abstimmungen oder Antwort‑Threading. Jede Voreinstellung kann über `config` überschrieben werden.
- `FastCommentsFeed` – ein sozialer Feed mit Beitrag‑Composer, Medien, Reaktionen, Folgen und Live‑Banner für neue Beiträge.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Das Standard‑Design wird aus einem Satz semantischer Design‑Tokens (`FastCommentsTheme`) generiert: Farben, Abstände, Radius, Schriftgrößen, Schriftstärken und Avatar‑Größen. Übergebe teilweise Token‑Überschreibungen (Typ `FastCommentsThemeOverrides`) über das `theme`‑Prop eines beliebigen Widgets, und der gesamte Stil‑Baum wird konsistent neu gestylt:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Der Dark‑Mode ist nur ein Token‑Set entfernt:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Das `styles`‑Prop akzeptiert weiterhin einen rohen `IFastCommentsStyles`‑Baum für präzise Kontrolle. Wenn sowohl `theme` als auch `styles` übergeben werden, haben die expliziten Styles Vorrang vor dem thematisierten Baum; wenn nur `styles` angegeben ist, ersetzt es die Standardwerte vollständig (das ursprüngliche Verhalten, sodass bestehende Integrationen und Skins unverändert bleiben). `setupDarkModeSkin` ist zugunsten des `theme`‑Props veraltet.

### Konfigurationsoptionen

Diese Bibliothek soll alle Konfigurationsoptionen unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind, genauso wie die Web‑Implementierung.

Zusätzlich dazu fügt React Native einige SDK‑spezifische Optionen über `FastCommentsRNConfig` hinzu:

- `hideTopBar` – blendet die Leiste mit dem angemeldeten Benutzer / Benachrichtigungsglocke aus, die über dem Composer angezeigt wird.
- `usePressToEdit` – ein Kommentar durch langes Drücken öffnen, um das Menü anzuzeigen.
- `disableDownVoting` – blendet die Down‑Vote‑Buttons aus.
- `renderCommentInline` – rendert die Kommentarinformationen im selben HTML‑Block wie der Kommentarinhalt.
- `renderLikesToRight` – verschiebt den Abstimm‑/Like‑Bereich rechts vom Kommentar statt darunter.
- `renderDateBelowComment` – rendert das Datum unterhalb des Kommentars.
- `showLiveStatus` – zeigt das Chat‑artige „Live“ + Benutzer‑Zähler‑Header‑Band über den Kommentaren.
- `useInlineSubmitButton` – rendert die Senden‑Schaltfläche als Icon innerhalb des Composers.
- `countAboveToggle` – in Kombination mit `useShowCommentsToggle`, wie viele Kommentare über dem „Show Comments“-Umschalter angezeigt werden.
- `preserveFeedScrollPosition` – `FastCommentsFeed` merkt sich den Bildlauf‑Offset über Unmount/Remount hinweg (Standard true).

### FastComments-Konzepte

Die wichtigsten Konzepte, die man für den Einstieg kennen sollte, sind `tenantId` und `urlId`. `tenantId` ist die Kennung Ihres FastComments.com‑Kontos. `urlId` ist das Ziel, an das Kommentar‑Threads gebunden werden. Das kann eine Seiten‑URL, eine Produkt‑ID, eine Artikel‑ID usw. sein.

### Lokalisierung

Alle benutzerorientierten Texte in diesen Widgets (Button‑Beschriftungen, Platzhalter, leere Zustände, relative Daten wie „vor 5 Minuten“, Fehlermeldungen usw.) sind **serverseitig gesteuert**. Die Komponenten enthalten keine fest codierten englischen Zeichenketten; sie geben die Übersetzungen aus, die FastComments für das angeforderte Locale bereitstellt.

Um ein Locale anzufordern, setzen Sie `locale` in Ihrer Konfiguration:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Wenn kein `locale` gesetzt ist, liefert FastComments die Standardsprache des Tenants.

**Bearbeiten des Textes:** Übersetzungen werden im FastComments‑Dashboard verwaltet, nicht in diesem SDK. Um Formulierungen zu ändern, überschreiben Sie die Standard‑Texte oder fügen Sie eine Sprache hinzu, indem Sie die Übersetzungen für Ihr Konto im Dashboard bearbeiten – die Änderung wird von den Widgets automatisch übernommen, ohne dass ein App‑Release nötig ist. Das SDK liefert keine englischen Fallbacks; jeder Schlüssel, den Sie im Dashboard leer lassen, wird leer gerendert. Halten Sie die Schlüssel für jedes unterstützte Locale ausgefüllt.

### Benutzerbenachrichtigungen

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Benachrichtigungs‑/Kommentar‑Ebene abbestellt werden und unterstützen abonnementsbasierte Seiten‑Level‑Abonnements, sodass Nutzer Threads einer bestimmten Seite oder eines Artikels abonnieren können.

Zum Beispiel ist es möglich, Secure SSO zu verwenden, um den Nutzer zu authentifizieren und dann periodisch nach ungelesenen Benachrichtigungen zu pollen und sie an den Nutzer zu pushen.

Siehe [das Beispiel AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) für das Abrufen und Übersetzen ungelesener Benutzer‑Benachrichtigungen.

### Gif-Browser

Standardmäßig ist keine Bild‑ oder Gif‑Auswahl aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) für die Unterstützung von Bild‑ und Gif‑Uploads. Es gibt einen Gif‑Browser, der Suchen und Bilder in dieser Bibliothek anonymisiert; Sie müssen ihn lediglich verwenden.

### Leistung

Bitte eröffnen Sie ein Ticket mit einem reproduzierbaren Beispiel, einschließlich des verwendeten Geräts, falls Sie Leistungsprobleme feststellen. Leistung ist ein zentrales Anliegen aller FastComments‑Bibliotheken.