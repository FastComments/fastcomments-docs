Live-Threaded-Kommentierung mit Avataren, verschachtelten Antworten, Stimmen und dem integrierten Rich‑Text‑Composer, plus einem dunklen Design und einer Live‑Chat‑Voreinstellung (hier gerendert über `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live‑Kommentare</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live‑Kommentare, helles Design"/></td>
    <td align="center"><b>Dunkles Design</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live‑Kommentare, dunkles Design"/></td>
    <td align="center"><b>Live‑Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live‑Chat‑Voreinstellung"/></td>
  </tr>
</table>

### Rich‑Text‑Editor

Diese Bibliothek verwendet [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) für Rich‑Text‑Bearbeitung, die ein leistungsstarkes WYSIWYG‑Erlebnis bietet. Derselbe Editor treibt iOS, Android und das Web (via `react-native-web`) an, sodass sich der Composer auf jeder Plattform konsistent mit einer einzigen Implementierung verhält.

`react-native-enriched` erfordert die React‑Native‑Neue‑Architektur (Fabric) nativ (Standard seit RN 0.76, optional seit RN 0.72‑0.75) und einen Bundler, der Paket‑`exports`‑Bedingungen auflöst. Dieses SDK wurde gegen RN 0.81 / React 19 entwickelt und getestet. Derselbe Editor läuft auch im Web über `react-native-web`; der Web‑Build des Enriched‑Editors ist upstream noch als experimentell gekennzeichnet.

### Widgets

- `FastCommentsLiveCommenting` – Threaded‑Kommentare mit Stimmen, Antworten, Paginierung, Erwähnungen, Benachrichtigungen und Live‑Updates.
- `FastCommentsLiveChat` – eine Chat‑Voreinstellung über dieselbe Engine: chronologische Nachrichten, wobei neue unten erscheinen, der Composer unter der Liste, ein Live‑Header‑Band (Verbindungs‑Punkt + Benutzeranzahl), unendliche Historie, die beim Hochscrollen geladen wird, automatisches Scrollen zu neuen Nachrichten, keine Stimmen oder Antwort‑Threading. Jede Voreinstellung kann über `config` überschrieben werden.
- `FastCommentsFeed` – ein sozialer Feed mit Beitrags‑Composer, Medien, Reaktionen, Followern und Live‑Bannern für neue Beiträge.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Der Standard‑Look wird aus einem Satz semantischer Design‑Tokens (`FastCommentsTheme`) generiert: Farben, Abstände, Radius, Schriftgrößen, Schriftgewichte und Avatar‑Größen. Übergebe partielle Token‑Überschreibungen (Typ `FastCommentsThemeOverrides`) über die `theme`‑Prop eines Widgets, und der gesamte Stil‑Baum wird konsistent neu gestylt:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark‑Mode ist nur ein Token‑Set entfernt:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Die `styles`‑Prop akzeptiert weiterhin einen rohen `IFastCommentsStyles`‑Baum für chirurgische Kontrolle. Wenn sowohl `theme` als auch `styles` bereitgestellt werden, haben die expliziten Styles Vorrang vor dem thematisierten Baum; wenn nur `styles` bereitgestellt wird, ersetzt es die Vorgaben vollständig (das ursprüngliche Verhalten, sodass bestehende Integrationen und Skins unbeeinflusst bleiben). `setupDarkModeSkin` ist zugunsten der `theme`‑Prop veraltet.

### Konfigurationsoptionen

Diese Bibliothek zielt darauf ab, alle Konfigurationsoptionen zu unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind – genau wie die Web‑Implementierung.

Zusätzlich fügt React Native einige SDK‑spezifische Optionen über `FastCommentsRNConfig` hinzu:

- `hideTopBar` – versteckt die Leiste mit dem angemeldeten Benutzer / Benachrichtigungsglocke, die über dem Composer angezeigt wird.
- `usePressToEdit` – drücken und halten eines Kommentars, um sein Menü zu öffnen.
- `disableDownVoting` – versteckt die Down‑Vote‑Schaltflächen.
- `renderCommentInline` – rendert die Kommentator‑Informationen im selben HTML‑Block wie den Kommentarinhalt.
- `renderLikesToRight` – verschiebt den Stimmen‑/Like‑Bereich nach rechts vom Kommentar statt darunter.
- `renderDateBelowComment` – rendert das Datum unter dem Kommentar.
- `showLiveStatus` – zeigt das Chat‑ähnliche „Live“ + Benutzeranzahl‑Header‑Band über den Kommentaren.
- `useInlineSubmitButton` – rendert die Senden‑Schaltfläche als Symbol im Composer.
- `countAboveToggle` – bei `useShowCommentsToggle`, wie viele Kommentare über dem „Kommentare anzeigen“-Umschalter gerendert werden.
- `preserveFeedScrollPosition` – `FastCommentsFeed` merkt sich seinen Bildlauf‑Offset über Unmount/Remount hinweg (Standard: true).

### FastComments‑Konzepte

Die wichtigsten Konzepte, die man kennen muss, um loszulegen, sind `tenantId` und `urlId`. `tenantId` ist die Kennung Ihres FastComments.com‑Kontos. `urlId` ist das Element, an das Kommentar‑Threads gebunden werden. Das kann eine Seiten‑URL, eine Produkt‑ID, eine Artikel‑ID usw. sein.

### Lokalisierung

Alle benutzer‑sichtbaren Texte in diesen Widgets (Button‑Beschriftungen, Platzhalter, leere Zustände, relative Daten wie „vor 5 Minuten“, Fehlermeldungen usw.) sind **servergesteuert**. Die Komponenten codieren keine englischen Strings fest; sie rendern die Übersetzungen, die FastComments für die angeforderte Locale bereitstellt.

Um eine Locale anzufordern, setzen Sie `locale` in Ihrer Config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Wenn keine `locale` gesetzt ist, liefert FastComments die Standardsprache des Tenants.

**Text bearbeiten:** Übersetzungen werden im FastComments‑Dashboard verwaltet, nicht in diesem SDK. Um Formulierungen zu ändern, überschreiben Sie den Standard‑Copy oder fügen Sie eine Sprache hinzu, bearbeiten Sie die Übersetzungen für Ihr Konto im Dashboard – die Änderung wird von den Widgets automatisch übernommen, ohne dass ein App‑Release nötig ist. Das SDK liefert keine englischen Fallbacks, sodass jeder Schlüssel, den Sie im Dashboard leerlassen, leer gerendert wird; halten Sie die Schlüssel für jede unterstützte Locale befüllt.

### Benutzerbenachrichtigungen

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Benachrichtigungs‑/Kommentar‑Ebene abbestellt werden und unterstützen seiten‑weite Abonnements, sodass Nutzer Threads einer bestimmten Seite oder eines Artikels abonnieren können.

Zum Beispiel ist es möglich, Secure SSO zu nutzen, um den Nutzer zu authentifizieren und dann periodisch nach ungelesenen Benachrichtigungen zu pollen und sie dem Nutzer zu pushen.

Siehe [das Beispiel AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) für die Vorgehensweise zum Abrufen und Übersetzen ungelesener Nutzer‑Benachrichtigungen.

### Gif‑Browser

Standardmäßig ist keine Bild‑ oder Gif‑Auswahl aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) für die Unterstützung von Bild‑ und Gif‑Uploads. Es gibt einen Gif‑Browser, der Suchvorgänge und Bilder in dieser Bibliothek anonymisiert; Sie müssen ihn lediglich verwenden.

### Leistung

Bitte öffnen Sie ein Ticket mit einem reproduzierbaren Beispiel, inklusive des verwendeten Geräts, falls Sie Leistungsprobleme feststellen. Leistung ist ein erstklassiger Aspekt aller FastComments‑Bibliotheken.