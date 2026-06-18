Deze bibliotheek is een complete react-native implementatie van [FastComments](https://fastcomments.com).

Het ondersteunt live commenting, chat, threads, emoticons, notifications, SSO, skins en volledige aanpassing door een stylesheet-object mee te geven. Alle assets
kunnen ook worden aangepast, en het ondersteunt het schakelen tussen verschillende assets op basis van de donkere modus.

Het voordeel van deze bibliotheek is dat het flexibeler is dan de `fastcomments-react-native` wrapper. Comments worden gerenderd met native components in plaats van binnen een webview.

Het draait allemaal op de FastComments backend, dus je hoeft alleen de UI te integreren:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zie [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) voor meer voorbeelden.

Voeg live chat toe aan je bestaande React Native-applicatie, of bouw zelfs een sociaal netwerk!