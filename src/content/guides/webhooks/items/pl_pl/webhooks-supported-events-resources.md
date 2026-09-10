---
FastComments obsługuje webhooki tylko dla zasobu Comment.

Obsługujemy webhooki dla tworzenia komentarzy, ich usuwania oraz aktualizacji.

Każdy z nich jest traktowany jako osobne zdarzenie w naszym systemie i dlatego ma różne semantyki
i struktury zdarzeń webhooków.

Dowolna liczba endpointów może subskrybować to samo zdarzenie: jeden webhook na domenę może być skonfigurowany w
panelu sterowania, a kolejne subskrypcje mogą być tworzone poprzez API (zobacz Managing Webhooks via the API).
---