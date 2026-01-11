### Identyfikatory transmisji

Zobaczysz, że w niektórych wywołaniach API powinieneś przekazać `broadcastId`. Gdy otrzymasz zdarzenia, otrzymasz ten identyfikator z powrotem, dzięki czemu będziesz wiedzieć, aby zignorować zdarzenie, jeśli planujesz optymistycznie zastosować zmiany po stronie klienta (czego prawdopodobnie będziesz chciał zrobić, ponieważ daje to najlepsze doświadczenie). Przekaż tutaj UUID. Identyfikator powinien być wystarczająco unikalny, aby nie występował dwukrotnie w trakcie sesji.

```swift
let broadcastId = UUID().uuidString
```