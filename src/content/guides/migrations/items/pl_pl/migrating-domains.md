FastComments udostępnia zautomatyzowany sposób migracji komentarzy między domenami.

Migracja domen wymaga jedynie domeny `from` i domeny `to`.

To **przenosi** komentarze, nie kopiuje ich. Jeśli chcesz skopiować komentarze, skontaktuj się z nami.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; title='Migrating Domains' app-screenshot-end]

Jest to również przydatne, na przykład jeśli część migracji do FastComments polega na przejściu z innego dostawcy, więc dane importu komentarzy
mogą zawierać elementy wymagające migracji. W takim przypadku możesz wykonać import, a następnie migrację domen.

### Monitorowanie postępu

Narzędzie migracji domen korzysta z tego samego systemu przetwarzania zadań FastComments co pozostałe narzędzia do zarządzania danymi.

Może wystąpić opóźnienie przed rozpoczęciem migracji. To normalne, ponieważ system okresowo sprawdza nowe zadania do przetworzenia.

W miarę wykonywania zadania wyświetlana będzie liczba komentarzy znalezionych do migracji oraz liczba dotychczas przeniesionych.

---