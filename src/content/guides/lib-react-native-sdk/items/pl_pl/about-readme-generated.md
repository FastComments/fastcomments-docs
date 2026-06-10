Ta biblioteka jest kompletną implementacją react-native dla [FastComments](https://fastcomments.com).

Obsługuje komentowanie na żywo, czat, wątki, emotikony, powiadomienia, SSO, motywy oraz pełną personalizację poprzez przekazanie obiektu stylesheet. Wszystkie zasoby
mogą być również dostosowywane, a biblioteka obsługuje przełączanie różnych zasobów w zależności od trybu ciemnego.

Zaletą tej biblioteki jest to, że jest bardziej elastyczna niż wrapper `fastcomments-react-native`. Komentarze są renderowane za pomocą natywnych komponentów zamiast wewnątrz webview.

Wszystko działa na backendzie FastComments, więc musisz jedynie zaimplementować UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zobacz [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) aby uzyskać więcej przykładów.

Dodaj czat na żywo do istniejącej aplikacji React Native, albo nawet zbuduj sieć społecznościową!