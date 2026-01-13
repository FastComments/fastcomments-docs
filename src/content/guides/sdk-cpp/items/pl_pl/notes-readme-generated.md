### Identyfikatory Broadcast

Zauważysz, że w niektórych wywołaniach API trzeba przekazać `broadcastId`. Gdy otrzymasz zdarzenia, otrzymasz z powrotem ten identyfikator, dzięki czemu będziesz mógł zignorować zdarzenie, jeśli planujesz optymistycznie zastosować zmiany po stronie klienta
(na co prawdopodobnie zechcesz się zdecydować, ponieważ zapewnia to najlepsze doświadczenie). Przekaż tutaj UUID. Identyfikator powinien być na tyle unikalny, aby nie wystąpił dwukrotnie w sesji przeglądarki.

### SSO (logowanie jednokrotne)

Przykłady SSO znajdziesz poniżej.