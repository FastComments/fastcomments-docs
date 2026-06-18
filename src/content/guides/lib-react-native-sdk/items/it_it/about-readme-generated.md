Questa libreria è un'implementazione completa per react-native di [FastComments](https://fastcomments.com).

Supporta commenti in tempo reale, chat, thread, emoticon, notifiche, SSO, skin e la piena personalizzazione passando un oggetto stylesheet. Tutte le risorse
possono anche essere personalizzate, e supporta l'attivazione di asset diversi in base alla modalità scura.

Il vantaggio di questa libreria è che è più flessibile rispetto al wrapper `fastcomments-react-native`. I commenti vengono renderizzati con componenti nativi anziché all'interno di una webview.

Tutto funziona sul backend di FastComments, quindi devi solo integrare l'interfaccia utente:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Vedi [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) per altri esempi.

Aggiungi una chat in tempo reale alla tua applicazione React Native esistente, o crea persino un social network!