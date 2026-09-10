## Wyzwalacze

Wyzwalacze uruchamiają Zapa, gdy w FastComments wydarzy się coś. Wszystkie trzy są natychmiastowe: FastComments dostarcza zdarzenie do Zapiera za pośrednictwem webhooka w momencie jego wystąpienia. Nic nie odpyta twojego konta i nie są zużywane żadne kredyty API podczas oczekiwania.

| Wyzwalacz | Uruchamia się gdy |
|-----------|-------------------|
| Nowy komentarz | Komentarz zostaje opublikowany. Domyślnie wyzwalane są tylko zatwierdzone, nie‑spamowe komentarze. |
| Zaktualizowany komentarz | Komentarz jest edytowany, zatwierdzany, oceniany, przypinany, blokowany lub w inny sposób zmieniany. |
| Usunięty komentarz | Komentarz zostaje usunięty. |

Każdy wyzwalacz zwraca pełny komentarz: identyfikator, adres URL strony i identyfikator URL, imię i e‑mail komentującego, treść komentarza jako markdown i jako HTML, liczbę głosów, flagi zatwierdzenia i spamu, ustawienia regionalne, domenę oraz wszelkie wzmianki. Pola odpowiadają ładunkowi webhooka udokumentowanemu w sekcji Webhooks, Struktury danych.

## Opcje

**Domena.** Każdy wyzwalacz ma opcjonalny filtr domeny, wymieniający domeny skonfigurowane na twoim koncie. Pozostaw puste, aby otrzymywać zdarzenia ze wszystkich domen.

**Uwzględnij niezatwierdzone i spamowe komentarze.** Tylko w wyzwalaczu Nowy komentarz. Komentarze, które są wstrzymane do moderacji lub oznaczone jako spam, są domyślnie pomijane. Gdy taki komentarz zostanie później zatwierdzony, wyzwalacz Zaktualizowany komentarz uruchamia się dla niego, więc Zap, który ma reagować na każdy komentarz stający się widoczny, używa Zaktualizowanego komentarza z filtrem na polu zatwierdzonym.

## Jak działa dostarczanie

Włączenie Zapa tworzy subskrypcję webhooka na twoim koncie, widoczną na stronie Webhooks ze źródłem **API**. Wyłączenie Zapa usuwa ją. Własne limity Zapiera dotyczą liczby zdarzeń, które akceptuje na minutę; FastComments ponawia dostarczenie, które się nie powiodło, z rosnącym opóźnieniem, i wyłącza subskrypcję, która nieustannie zawodzi przez sześć dni. Wyłączona subskrypcja może zostać ponownie włączona ze strony Webhooks, lub po prostu wyłącz i włącz Zapa ponownie, aby utworzyć nową.

Konto może posiadać do 50 subskrypcji API. Każdy Zap korzystający z wyzwalacza FastComments używa jednej.