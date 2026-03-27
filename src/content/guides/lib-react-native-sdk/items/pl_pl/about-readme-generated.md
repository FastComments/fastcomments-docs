Ta biblioteka jest kompletną implementacją react-native [FastComments](https://fastcomments.com).

Obsługuje komentowanie na żywo, chat, wątki, emotikony, powiadomienia, SSO, skórki oraz pełną personalizację poprzez przekazanie obiektu arkusza stylów. Wszystkie zasoby
mogą być również dostosowywane, a biblioteka obsługuje przełączanie różnych zasobów w zależności od trybu ciemnego.

Zaletą tej biblioteki jest to, że jest bardziej elastyczna niż wrapper `fastcomments-react-native`. Komentarze są renderowane za pomocą natywnych komponentów, a nie wewnątrz webview. Uwaga: `react-native-webview` jest nadal wymagana jako zależność pośrednia dla edytora tekstu sformatowanego (`@10play/tentap-editor`).

Wszystko działa na backendzie FastComments, więc musisz jedynie zaimplementować interfejs użytkownika (UI):

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Zobacz [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) aby uzyskać więcej przykładów.

Dodaj czat na żywo do istniejącej aplikacji React Native albo nawet zbuduj sieć społecznościową!