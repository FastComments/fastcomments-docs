### Identyfikatory transmisji

Zauważysz, że w niektórych wywołaniach API należy przekazać `broadcastId`. Gdy otrzymasz zdarzenia, otrzymasz z powrotem ten identyfikator, więc będziesz wiedzieć, aby zignorować zdarzenie, jeśli planujesz optymistycznie zastosować zmiany po stronie klienta
(czego prawdopodobnie będziesz chciał zrobić, ponieważ daje to najlepsze doświadczenie). Przekaż tutaj UUID. Identyfikator powinien być na tyle unikalny, by nie wystąpić dwa razy w sesji przeglądarki.

### SSO (Logowanie jednokrotne)

Przykłady SSO znajdziesz poniżej.