FastComments to platforma z obsługą lokalizacji. Wszystkie nasze widgety, e-maile i powiadomienia są zlokalizowane.

Lokalizacja oznacza, że wyświetlamy inny język i formatowanie w zależności od lokalizacji użytkownika i preferowanego języka. Określamy to na podstawie informacji dostarczonych przez przeglądarkę użytkownika.

Możemy dostosować tekst w e-mailu, przechodząc do zakładki `Translations`, wybierając `Locale`
i edytując tekst. Tekst zmieniony względem domyślnego jest wyróżniony w interfejsie. Możesz
przełączać się między lokalizacjami i zapisać na końcu, nie tracąc zmian.

Tekst zlokalizowany jest dostępny przez obiekt `TEXT`, na przykład: `<%= TEXT.INTRO %>`.

### Uwaga dotycząca SSO

W integracjach SSO, jeśli `locale` nie jest określony, będzie on aktualizowany za każdym razem, gdy użytkownik
uzyska dostęp do widżetu komentarzy z innym locale. Oznacza to, że jego preferencje językowe
są aktualizowane automatycznie, a przyszłe e-maile będą wysyłane w tym locale.

Można to również ustawić ręcznie, podając `locale` w payloadzie SSO.