Ta biblioteka to kompletna implementacja react-native dla [FastComments](https://fastcomments.com).

Obsługuje komentarze na żywo, czat, wątki, emotikony, powiadomienia, SSO, skórki oraz pełną personalizację poprzez przekazanie obiektu stylesheet. Wszystkie zasoby również można dostosować, a biblioteka obsługuje przełączanie różnych zasobów w zależności od trybu ciemnego.

Zaletą tej biblioteki jest większa elastyczność i brak konieczności używania webview, w przeciwieństwie do wrappera `fastcomments-react-native`.

Wszystko działa na backendzie FastComments, więc musisz jedynie zaimplementować interfejs użytkownika:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zobacz [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) aby uzyskać więcej przykładów.

Dodaj czat na żywo do istniejącej aplikacji React Native, a nawet zbuduj sieć społecznościową!