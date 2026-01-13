### Identyfikatory Broadcast

Zauważysz, że w niektórych wywołaniach API należy przekazać `broadcastId`. Gdy otrzymasz zdarzenia, otrzymasz ten identyfikator z powrotem, dzięki czemu będziesz wiedzieć, aby zignorować zdarzenie, jeśli planujesz optymistycznie zastosować zmiany po stronie klienta
(co prawdopodobnie zechcesz zrobić, ponieważ zapewnia to najlepsze wrażenia). Przekaż tutaj UUID. Identyfikator powinien być na tyle unikalny, by nie wystąpić dwukrotnie w sesji przeglądarki.

### SSO (Logowanie jednokrotne)

Przykłady SSO znajdują się poniżej.