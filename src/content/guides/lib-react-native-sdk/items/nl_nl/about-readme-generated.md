Deze bibliotheek is een volledige react-native implementatie van [FastComments](https://fastcomments.com).

Het ondersteunt live reacties, chat, threads, emoticons, meldingen, SSO, skins, en volledige aanpassing door een stylesheet-object door te geven. Alle assets
kunnen ook aangepast worden, en het ondersteunt het wisselen van verschillende assets afhankelijk van de donkere modus.

Het voordeel van deze bibliotheek is dat hij flexibeler is en geen webview vereist, zoals de `fastcomments-react-native` wrapper.

Alles draait op de FastComments-backend, dus je hoeft alleen de UI te integreren:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zie [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) voor meer voorbeelden.

Voeg live chat toe aan je bestaande React Native-applicatie, of bouw zelfs een sociaal netwerk!