Skrypt dla każdego rozszerzenia jest pobierany i uruchamiany zanim widżet komentarzy zacznie pobierać pierwszy zestaw komentarzy i renderować UI.

Na początkowym ładowaniu następujące dane zostaną przypisane do obiektu rozszerzenia:

- `config` - Odwołanie do obiektu `config`.
- `translations` - Odwołanie do obiektu `translations`.
- `commentsById` - Odwołanie do wszystkich komentarzy według id.
- `root` - Odwołanie do głównego węzła DOM.

Rozszerzenia powinny nadpisać pożądane funkcje, które widżet komentarzy wywoła w odpowiednich momentach.