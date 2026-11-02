Każda witryna WordPress potrzebuje własnego konta FastComments lub najemcy. WordPress numeruje wpisy osobno na każdej witrynie, więc wpis 5 w jednej witrynie i wpis 5 w innej  
dzieliłby jeden wątek komentarzy, jeśli obie byłyby podłączone do tego samego konta. Z tego powodu konfiguracja wtyczki odmawia podłączenia witryny, gdy konto ma już podłączoną inną.

[Możesz zobaczyć swoje aktywne integracje tutaj.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Aby obsługiwać kilka witryn WordPress, utwórz [najemcę](https://fastcomments.com/auth/my-account/tenants) dla każdej z nich, przełącz się na niego, a następnie podłącz witrynę z strony konfiguracji wtyczki. Najemcy są wliczeni w plany z białą etykietą. Możesz także utworzyć osobne konto FastComments dla każdej witryny, używając innego adresu e‑mail dla każdej.

Jeśli witryna została przeniesiona, na przykład ze środowiska testowego do produkcji, dezaktywuj wtyczkę na starej witrynie i usuń starą witrynę z aktywnych integracji. Następnie podłącz nową. Komentarze już zsynchronizowane ze starą witryną pozostają na koncie i wyświetlają się przy każdym wpisie o tym samym ID na nowej witrynie.

Możesz używać tego samego konta FastComments z jedną instalacją WordPress, która obejmuje wiele domen i ma tę samą treść na tych domenach.