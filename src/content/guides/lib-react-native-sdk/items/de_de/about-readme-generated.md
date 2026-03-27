Diese Bibliothek ist eine vollständige react-native-Implementierung von [FastComments](https://fastcomments.com).

Sie unterstützt Live-Kommentare, Chat, Threads, Emojis, Benachrichtigungen, SSO, Skins und vollständige Anpassung durch Übergeben eines Stylesheet-Objekts. Alle Assets
können ebenfalls angepasst werden, und sie unterstützt das Umschalten verschiedener Assets basierend auf dem Dunkelmodus.

Der Vorteil dieser Bibliothek besteht darin, dass sie flexibler ist als der Wrapper `fastcomments-react-native`. Kommentare werden mit nativen Komponenten gerendert, statt in einer Webview. Hinweis: `react-native-webview` wird weiterhin als transitive Abhängigkeit des Rich-Text-Editors (`@10play/tentap-editor`) benötigt.

Alles läuft auf dem FastComments-Backend, sodass Sie nur die Benutzeroberfläche integrieren müssen:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Siehe [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) für weitere Beispiele.

Fügen Sie Live-Chat zu Ihrer bestehenden React Native-Anwendung hinzu oder bauen Sie sogar ein soziales Netzwerk!