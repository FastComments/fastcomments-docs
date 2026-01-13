Dette bibliotek er en komplet react-native-implementering af [FastComments](https://fastcomments.com).

Det understøtter live-kommentarer, chat, tråde, emoticons, notifikationer, SSO, skins og fuld tilpasning ved at sende et stylesheet-objekt. Alle assets
kan også tilpasses, og det understøtter at skifte mellem forskellige assets baseret på dark mode.

Fordelen ved dette bibliotek er, at det er mere fleksibelt, og ikke kræver en webview, som `fastcomments-react-native` wrapperen gør.

Det kører alt sammen på FastComments-backenden, så du skal kun integrere UI'et:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Se [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) for flere eksempler.

Tilføj live chat til din eksisterende React Native-applikation, eller byg endda et socialt netværk!