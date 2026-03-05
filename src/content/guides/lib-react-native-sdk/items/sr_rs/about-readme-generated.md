Ova biblioteka је potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava live komentarisanje, chat, threads, emoticons, notifications, SSO, skins, и potpunu prilagodbu прослеђивањем stylesheet objekta. Svi assets
takođe se mogu prilagoditi, и podržava prebacivanje različitih assets у зависности од dark mode.

Prednost ove biblioteke je што је флексибилнија, и не захтева webview, као `fastcomments-react-native` wrapper.

Sve radi на FastComments backend-u, тако да само треба да интегришете UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примера.

Dodajte live chat u vašu postojeću React Native aplikaciju, или čak izgradite društvenu mrežu!