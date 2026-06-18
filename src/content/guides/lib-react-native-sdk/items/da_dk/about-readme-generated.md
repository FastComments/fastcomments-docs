Dette bibliotek er en komplet react-native-implementering af [FastComments](https://fastcomments.com).

Det understøtter livekommentering, chat, tråde, emoticons, notifikationer, SSO, skins og fuld tilpasning ved at sende et stylesheet object. Alle assets
kan også tilpasses, og det understøtter at skifte mellem forskellige assets baseret på dark mode.

Fordelen ved dette bibliotek er, at det er mere fleksibelt end `fastcomments-react-native` wrapper. Kommentarer gengives med native komponenter i stedet for inde i en webview.

Det hele kører på FastComments backend, så du kun skal integrere UI'et:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Se [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) for flere eksempler.

Tilføj live chat til din eksisterende React Native application, eller byg endda et socialt netværk!