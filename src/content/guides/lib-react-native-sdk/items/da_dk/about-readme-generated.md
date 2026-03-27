Dette bibliotek er en komplet react-native-implementering af [FastComments](https://fastcomments.com).

Det understøtter live-kommentarer, chat, tråde, emoticons, notifikationer, SSO, skins og fuld tilpasning ved at give et stilark-objekt. Alle assets
kan også tilpasses, og det understøtter at skifte mellem forskellige assets baseret på mørkt tema.

Fordelen ved dette bibliotek er, at det er mere fleksibelt end `fastcomments-react-native`-wrapperen. Kommentarer gengives med native komponenter i stedet for inde i en webview. Bemærk: `react-native-webview` kræves stadig som en transitiv afhængighed af rich text-editoren (`@10play/tentap-editor`).

Det hele kører på FastComments-backend'en, så du skal kun integrere UI'en:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Se [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) for flere eksempler.

Tilføj live chat til din eksisterende React Native-applikation, eller byg endda et socialt netværk!