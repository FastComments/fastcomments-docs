---
Jak każdy rozproszony system FastComments potrzebuje sposobu na blokowanie zasobów i procedur. Te blokady można monitorować za pomocą endpointu `/locks-in-progress`.

[Na przykład, oto endpoint na naszym shardzie w USA](https://fastcomments.com/locks-in-progress).

To może być przydatne, aby zobaczyć, dlaczego system jest zawieszony lub obciążony. Jeśli na przykład SRE chce sprawdzić, dlaczego system doświadcza wysokiego obciążenia CPU, może
sprawdzić ten endpoint, aby uzyskać nazwę nieprawidłowo działającego crona.

---