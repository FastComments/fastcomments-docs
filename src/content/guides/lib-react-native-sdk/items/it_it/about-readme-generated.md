---
Questa libreria è un'implementazione completa per react-native di [FastComments](https://fastcomments.com).

Supporta commenti in tempo reale, chat, thread, emoticon, notifiche, SSO, skin, e piena personalizzazione passando un oggetto stylesheet. Tutte le risorse
possono anche essere personalizzate, e supporta l'attivazione di risorse diverse in base alla modalità scura.

Il vantaggio di questa libreria è che è più flessibile, e non richiede una webview, come il wrapper `fastcomments-react-native`.

Tutto viene eseguito sul backend di FastComments, quindi devi soltanto integrare l'interfaccia utente:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Vedi [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) per altri esempi.

Aggiungi la chat in tempo reale alla tua applicazione React Native esistente, o anche costruisci un social network!
---