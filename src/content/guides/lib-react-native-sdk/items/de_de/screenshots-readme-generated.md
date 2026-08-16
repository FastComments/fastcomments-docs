Live-Thread-Kommentare mit Avataren, verschachtelten Antworten, Stimmen und dem integrierten Rich‑Text‑Composer, plus einem dunklen Theme und einer Live‑Chat‑Voreinstellung (hier gerendert über `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live‑Kommentare</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Live‑Kommentare, helles Theme"/></td>
    <td align="center"><b>Dunkles Theme</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Live‑Kommentare, dunkles Theme"/></td>
    <td align="center"><b>Live‑Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live‑Chat‑Voreinstellung"/></td>
  </tr>
</table>

### Rich‑Text‑Editor

Diese Bibliothek verwendet [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) für Rich‑Text‑Bearbeitung, die ein leistungsstarkes WYSIWYG‑Erlebnis bietet. Derselbe Editor treibt iOS, Android und das Web (via `react-native-web`) an, sodass der Composer auf jeder Plattform konsistent mit einer einzigen Implementierung funktioniert.

`react-native-enriched` erfordert die React‑Native‑Neue‑Architektur (Fabric) nativ (Standard seit RN 0.76, optional bei RN 0.72‑0.75) und einen Bundler, der Paket‑`exports`‑Bedingungen auflöst. Dieses SDK wird entwickelt und getestet gegen RN 0.81 / React 19. Der gleiche Editor läuft auch im Web über `react-native-web`; das Web‑Build des Enriched‑Editors ist upstream noch als experimentell gekennzeichnet.

### Widgets

Das SDK liefert drei Widgets, die das FastComments Android SDK spiegeln:

- `FastCommentsLiveCommenting` – Threaded‑Kommentare mit Stimmen, Antworten, Paginierung, Erwähnungen, Benachrichtigungen und Live‑Updates.
- `FastCommentsLiveChat` – Eine Chat‑Voreinstellung über dieselbe Engine: chronologische Nachrichten mit neuen am unteren Ende, der Composer unterhalb der Liste, ein Live‑Header‑Band (Verbindungspunkt + Benutzeranzahl), unendlicher Verlauf, der durch Hochscrollen geladen wird, automatisches Scrollen zu neuen Nachrichten, keine Stimmen oder Antwort‑Threading. Jede Voreinstellung kann über `config` überschrieben werden.
- `FastCommentsFeed` – Ein sozialer Feed mit Beitrags‑Composer, Medien, Reaktionen, Followern und Live‑Bannern für neue Beiträge.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Der Standard‑Look wird aus einem Satz semantischer Design‑Tokens (`FastCommentsTheme`) generiert: Farben, Abstände, Radius, Schriftgrößen, Schriftstärken und Avatar‑Größen. Übergebe partielle Token‑Überschreibungen (Typ `FastCommentsThemeOverrides`) über die `theme`‑Prop bei jedem Widget, und der gesamte Stil‑Baum wird konsistent neu gestylt:

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

Diese Bibliothek zielt darauf ab, alle Konfigurationsoptionen zu unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind, genau wie die Web‑Implementierung.

Zusätzlich dazu fügt React Native einige SDK‑spezifische Optionen über `FastCommentsRNConfig` hinzu:

- `hideTopBar` – Versteckt die Leiste mit dem angemeldeten Benutzer / Benachrichtigungsglocke, die über dem Composer angezeigt wird.
- `usePressToEdit` – Durch langes Drücken eines Kommentars das Menü öffnen.
- `disableDownVoting` – Versteckt die Down‑Vote‑Schaltflächen.
- `renderCommentInline` – Rendert die Kommentator‑Informationen im selben HTML‑Block wie der Kommentarinhalt.
- `renderLikesToRight` – Verschiebt den Stimmen‑/Like‑Bereich nach rechts neben den Kommentar statt darunter.
- `renderDateBelowComment` – Rendert das Datum unter dem Kommentar.
- `showLiveStatus` – Zeigt das Chat‑ähnliche „Live“ + Benutzeranzahl‑Header‑Band über den Kommentaren.
- `useInlineSubmitButton` – Rendert die Senden‑Schaltfläche als Symbol im Composer.
- `countAboveToggle` – Mit `useShowCommentsToggle` gibt an, wie viele Kommentare über dem „Kommentare anzeigen“-Umschalter gerendert werden.
- `preserveFeedScrollPosition` – `FastCommentsFeed` merkt sich seine Bildlaufposition über Unmount/Remount hinweg (Standard: true).

### FastComments‑Konzepte

Die wichtigsten Konzepte, die man kennen muss, um loszulegen, sind `tenantId` und `urlId`. `tenantId` ist die Kennung deines FastComments.com‑Kontos. `urlId` ist das Element, an das Kommentar‑Threads gebunden werden. Das kann eine Seiten‑URL, eine Produkt‑ID, eine Artikel‑ID usw. sein.

### Lokalisierung

Alle benutzer‑sichtbaren Texte in diesen Widgets (Button‑Beschriftungen, Platzhalter, leere Zustände, relative Daten wie „vor 5 Minuten“, Fehlermeldungen usw.) sind **servergesteuert**. Die Komponenten codieren keine englischen Strings fest ein; sie rendern die Übersetzungen, die FastComments für die angeforderte Locale bereitstellt.

Um eine Locale anzufordern, setze `locale` in deiner Config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Wenn keine `locale` gesetzt ist, liefert FastComments die Standardsprache des Tenants.

**Text bearbeiten:** Übersetzungen werden in deinem FastComments‑Dashboard verwaltet, nicht in diesem SDK. Um Formulierungen zu ändern, überschreibe den Standard‑Copy oder füge eine Sprache hinzu, bearbeite die Übersetzungen für dein Konto im Dashboard – die Änderung wird von den Widgets automatisch übernommen, ohne dass ein App‑Release nötig ist. Das SDK liefert keine englischen Fallbacks, sodass jeder Schlüssel, den du im Dashboard leer lässt, leer gerendert wird; halte die Schlüssel für jede unterstützte Locale befüllt.

### Benutzerbenachrichtigungen

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Benachrichtigungs‑/Kommentar‑Ebene abbestellt werden und unterstützen abonnement‑basierte Seiten‑Abonnements, sodass Benutzer Threads einer bestimmten Seite oder eines Artikels abonnieren können.

Zum Beispiel ist es möglich, Secure SSO zu verwenden, um den Benutzer zu authentifizieren und dann periodisch nach ungelesenen Benachrichtigungen zu pollen und sie dem Benutzer zu pushen.

Siehe [das Beispiel AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) für die Implementierung und Übersetzung ungelesener Benutzer‑Benachrichtigungen.

### Gif‑Browser

Standardmäßig ist keine Bild‑ oder Gif‑Auswahl aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) für die Unterstützung von Bild‑ und Gif‑Uploads. Es gibt einen Gif‑Browser, der Suchanfragen und Bilder in dieser Bibliothek anonymisiert; du musst ihn lediglich verwenden.

### Leistung

Bitte eröffne ein Ticket mit einem reproduzierbaren Beispiel, inklusive verwendetem Gerät, falls du Leistungsprobleme feststellst. Leistung ist ein erstklassiger Aspekt aller FastComments‑Bibliotheken.