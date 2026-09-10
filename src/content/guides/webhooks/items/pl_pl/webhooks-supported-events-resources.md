FastComments obsługuje webhooki tylko dla zasobu Comment.

Obsługujemy webhooki dla tworzenia komentarzy, ich usuwania oraz aktualizacji.

Każdy z nich jest traktowany jako osobne zdarzenie w naszym systemie i w związku z tym ma różne semantyki
i struktury zdarzeń webhooków.

Dowolna liczba endpointów może subskrybować to samo zdarzenie, z poziomu panelu sterowania lub poprzez API
(zobacz Managing Webhooks via the API). Każdy webhook jest dostarczany niezależnie.