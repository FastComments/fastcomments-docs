Diese Bibliothek ist eine vollständige react-native-Implementierung von [FastComments](https://fastcomments.com).

Sie unterstützt Live-Kommentare, Chat, Threads, Emoticons, Benachrichtigungen, SSO, Skins und vollständige Anpassung durch Übergabe eines Stylesheet-Objekts. Alle Assets
können ebenfalls angepasst werden, und es wird unterstützt, verschiedene Assets basierend auf dem Dunkelmodus umzuschalten.

Der Vorteil dieser Bibliothek ist, dass sie flexibler ist und kein WebView benötigt, wie der Wrapper `fastcomments-react-native`.

Alles läuft auf dem FastComments-Backend, sodass Sie nur die UI integrieren müssen:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Siehe [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) für weitere Beispiele.

Fügen Sie Live-Chat zu Ihrer bestehenden React Native-Anwendung hinzu oder bauen Sie sogar ein soziales Netzwerk!