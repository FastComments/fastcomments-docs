FastComments obsługuje automatyczny tryb konserwacji. Jeśli baza danych przestanie działać, może nadal serwować popularne wątki komentarzy.

Dodatkowo, w trybie konserwacji wszystkie komentarze są zapisywane w `BACKUP_DIR`. Zostaną one przetworzone (sprawdzone pod kątem spamu itd.) i zapisane, gdy system wróci online.

Robi to, co godzinę, określając 100 najpopularniejszych wątków komentarzy i buforując ich zawartość na dysku. Określanie 100 najlepszych wątków jest już wykonywane na podstawie wstępnie obliczonego stanu, więc nie jest to ciężkie zadanie periodyczne.

To jest całkowicie opcjonalne i włączane tylko jeśli ustawione są `CACHE_DIR` i `BACKUP_DIR`. Oczywiście sprawia to, że węzły aplikacji stają się stanowe, jednak jest to stan, który może zostać utracony w dowolnym momencie bez powodowania niewłaściwego działania aplikacji.

Należy pamiętać, że w trybie konserwacji nie da się przeprowadzić właściwej autoryzacji wątków komentarzy, dlatego okresowo tworzony jest backup tylko tych wątków, które można bezpiecznie uznać za publiczne.

W trybie konserwacji wiele funkcji jest niedostępnych.