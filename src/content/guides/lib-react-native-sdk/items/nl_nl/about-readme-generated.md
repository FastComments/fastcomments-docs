---
Deze bibliotheek is een complete react-native implementatie van [FastComments](https://fastcomments.com).

Het ondersteunt live commenting, chat, discussiedraden, emoticons, meldingen, SSO, skins, en volledige aanpassing door een stylesheet-object door te geven. Alle assets
kunnen ook aangepast worden, en het ondersteunt het schakelen tussen verschillende assets op basis van donkere modus.

Het voordeel van deze bibliotheek is dat hij flexibeler is dan de `fastcomments-react-native` wrapper. Reacties worden gerenderd met native componenten in plaats van binnen een webview. Opmerking: `react-native-webview` is nog steeds vereist als transitieve afhankelijkheid van de rich text editor (`@10play/tentap-editor`).

Het draait allemaal op de FastComments backend, dus je hoeft alleen de UI te integreren:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zie [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) voor meer voorbeelden.

Voeg live chat toe aan je bestaande React Native applicatie, of bouw zelfs een sociaal netwerk!
---