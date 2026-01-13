### Identyfikatory broadcastów

Zobaczysz, że w niektórych wywołaniach API powinieneś przekazać `broadcastId`. Kiedy otrzymasz zdarzenia, otrzymasz z powrotem ten identyfikator, dzięki czemu będziesz wiedział, aby zignorować zdarzenie jeśli planujesz optymistycznie zastosować zmiany po stronie klienta
(czego prawdopodobnie zechcesz zrobić, ponieważ daje to najlepsze doświadczenie). Przekaż tutaj UUID. Identyfikator powinien być na tyle unikalny, by nie pojawił się dwukrotnie w sesji przeglądarki.