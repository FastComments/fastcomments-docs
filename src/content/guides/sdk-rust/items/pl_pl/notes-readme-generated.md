### Identyfikatory transmisji

Zauważysz, że w niektórych wywołaniach API trzeba przekazać `broadcastId`. Kiedy otrzymasz zdarzenia, otrzymasz z powrotem to ID, dzięki czemu będziesz wiedzieć, aby zignorować zdarzenie, jeśli planujesz optymistycznie zastosować zmiany po stronie klienta
(czego prawdopodobnie będziesz chciał zrobić, ponieważ zapewnia to najlepsze doświadczenie). Przekaż tutaj UUID. ID powinno być na tyle unikalne, by nie pojawiło się dwa razy w sesji przeglądarki.