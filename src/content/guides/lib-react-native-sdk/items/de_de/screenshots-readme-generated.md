Echtzeit-Thread-Kommentare mit Avataren, verschachtelten Antworten, Stimmen und dem integrierten Rich-Text-Komponisten, dazu ein dunkles Theme und eine Live-Chat-Voreinstellung (hier gerendert via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Echtzeit-Kommentare</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Echtzeit-Kommentare, helles Theme"/></td>
    <td align="center"><b>Dunkles Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Echtzeit-Kommentare, dunkles Theme"/></td>
    <td align="center"><b>Live-Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live-Chat-Voreinstellung"/></td>
  </tr>
</table>

### Rich Text Editor

Diese Bibliothek verwendet [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) für die Rich-Text-Bearbeitung, die ein leistungsfähiges WYSIWYG-Bearbeitungserlebnis bietet. Derselbe Editor treibt iOS, Android und das Web (via `react-native-web`) an, sodass der Composer mit einer einzigen Implementierung plattformübergreifend konsistent verhält.

`react-native-enriched` erfordert die React Native New Architecture (Fabric) auf nativen Plattformen (seit RN 0.76 Standard, opt-in bei RN 0.72–0.75) sowie einen Bundler, der die Package-`exports`-Bedingungen auflöst. Dieses SDK wird gegen RN 0.81 / React 19 entwickelt und getestet. Der Web-Build des enriched-Editors wird upstream weiterhin als experimentell gekennzeichnet.

### Widgets

Das SDK enthält drei Widgets, die dem FastComments Android SDK entsprechen:

- `FastCommentsLiveCommenting` - Threaded-Kommentare mit Stimmen, Antworten, Paginierung, Erwähnungen, Benachrichtigungen und Live-Aktualisierungen.
- `FastCommentsLiveChat` - eine Chat-Voreinstellung auf derselben Engine: chronologische Nachrichten mit neuen Einträgen unten, Composer unterhalb der Liste, eine Live-Header-Leiste (Verbindungsdot + Nutzeranzahl), unendliche Historie, die durch Hochscrollen geladen wird, automatisches Scrollen zu neuen Nachrichten, keine Stimmen oder Antwort-Threads. Jede Voreinstellung kann über `config` überschrieben werden.
- `FastCommentsFeed` - ein Social-Feed mit Beitrags-Komponist, Medien, Reaktionen, Follows und Live-Bannern für neue Beiträge.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Das Standardaussehen wird aus einem Satz semantischer Designtokens (`FastCommentsTheme`) erzeugt: Farben, Abstände, Radius, Schriftgrößen, Schriftstärken und Avatar-Größen. Übergebe partielle Token-Overrides (typisiert als `FastCommentsThemeOverrides`) über die `theme`-Prop an ein beliebiges Widget, und der gesamte Stilbaum passt sich konsistent an:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dunkelmodus ist nur ein Token-Set entfernt:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Die `styles`-Prop akzeptiert weiterhin einen rohen `IFastCommentsStyles`-Baum für fein abgestimmte Kontrolle. Wenn sowohl `theme` als auch `styles` übergeben werden, haben die expliziten Styles Vorrang vor dem thematischen Baum; wenn nur `styles` übergeben wird, ersetzt es die Defaults vollständig (das ursprüngliche Verhalten, sodass bestehende Integrationen und Skins unbeeinflusst bleiben). `setupDarkModeSkin` ist veraltet zugunsten der `theme`-Prop.

### Configuration Options

Diese Bibliothek hat das Ziel, alle Konfigurationsoptionen zu unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind, genau wie die Web-Implementierung.

Darüber hinaus fügt React Native einige SDK-spezifische Optionen über `FastCommentsRNConfig` hinzu:

- `hideTopBar` - blendet die Leiste mit dem angemeldeten Nutzer / der Benachrichtigungsglocke aus, die oberhalb des Composers angezeigt wird.
- `usePressToEdit` - Kommentar gedrückt halten, um sein Menü zu öffnen.
- `disableDownVoting` - versteckt die Down-Vote-Schaltflächen.
- `renderCommentInline` - rendert Kommentatorinformationen innerhalb desselben HTML-Blocks wie der Kommentarinhalt.
- `renderLikesToRight` - verschiebt den Bereich für Stimmen/Likes rechts vom Kommentar statt darunter.
- `renderDateBelowComment` - rendert das Datum unter dem Kommentar.
- `showLiveStatus` - zeigt die chatartige "Live" + Nutzeranzahl-Header-Leiste über den Kommentaren.
- `useInlineSubmitButton` - rendert die Absende-Schaltfläche als Icon innerhalb des Composers.
- `countAboveToggle` - bei `useShowCommentsToggle`, wie viele Kommentare über dem "Kommentare anzeigen"-Toggle gerendert werden.
- `preserveFeedScrollPosition` - `FastCommentsFeed` merkt sich seine Scroll-Position über Unmount/Remount hinweg (Standard true).

### FastComments Concepts

Die wichtigsten Konzepte, die für den Einstieg zu beachten sind, sind `tenantId` und `urlId`. `tenantId` ist Ihr FastComments.com Konto-Identifier. `urlId` gibt an, womit Kommentar-Threads verknüpft werden sollen. Das kann eine Seiten-URL, eine Produkt-ID, eine Artikel-ID usw. sein.

### User Notifications

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Benachrichtigungs-/Kommentar-Ebene abgewählt werden und unterstützen seitenweite Abonnements, sodass Nutzer Threads einer bestimmten Seite oder eines Artikels abonnieren können.

Beispielsweise ist es möglich, Secure SSO zur Authentifizierung des Nutzers zu verwenden und dann periodisch auf ungelesene Benachrichtigungen zu prüfen und diese an den Nutzer zu pushen.

Siehe [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dafür, wie man ungelesene Nutzerbenachrichtigungen erhält und übersetzt.

### Gif Browser

Standardmäßig ist keine Bild- oder Gif-Auswahl aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dafür, wie man Bild- und Gif-Uploads unterstützt. In dieser Bibliothek gibt es einen Gif-Browser, der Suchanfragen und bereitgestellte Bilder anonymisiert — Sie müssen ihn nur verwenden.

### Performance

Bitte eröffnen Sie ein Ticket mit einem reproduzierbaren Beispiel, einschließlich verwendetem Gerät, wenn Sie Leistungsprobleme feststellen. Performance hat bei allen FastComments-Bibliotheken hohe Priorität.