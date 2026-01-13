### Wymagane komponenty

Dla On-Prem FastComments składa się tylko z serwera aplikacji i bazy danych. Uprościliśmy wdrożenie tak, aby aplikacja mogła obsługiwać cały ruch bezpośrednio, bez dodawania innych komponentów.

Serwer aplikacji jest dostarczany w obrazie Dockera i może być wdrożony przy użyciu dowolnego rozwiązania do zarządzania kontenerami.

Baza danych, MongoDB, może być uruchomiona samodzielnie lub hostowana przez innego dostawcę, np. AWS DocumentDB lub MongoDB Atlas.

FastComments jest obecnie testowany z MongoDB 7, jednak dążymy do zgodności z DocumentDB, aby ułatwić wdrożenie.

### Rozmiary instancji

Zauważysz, że FastComments jest dość dobrze zoptymalizowany i nie wymaga dużych maszyn dla samej aplikacji, aby utrzymać niskie wartości P99.

Wszystkie zadania wsadowe i cron korzystają ze strumieniowania, aby ograniczyć całkowite zużycie pamięci.

Poniższe tabele dla serwera aplikacji i bazy danych mogą pomóc przy doborze rozmiarów.

### Instancje serwera aplikacji


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Na przykład pojedyncze jądro obsługujące około 100 wątków komentarzy na sekundę zwykle nigdy nie używa więcej niż 250mb RSS.

### Instancje serwera bazy danych

Dobór rozmiaru bazy danych zależy od rozmiaru zestawu roboczego (working set), czyli ilości danych, do których uzyskujesz dostęp w danym momencie, a także od liczby równoczesnych żądań.

FastComments jest dość łagodny dla Mongo — dla gorących zapytań używa wskazówek indeksów, strumieniowych kursorów i ma limity współbieżności w różnych obszarach, aby zapobiec przeciążeniu systemów dalszego przetwarzania.

Poniżej znajduje się ogólna wskazówka dotycząca rozmiarów instancji bazy danych. **Należy pamiętać, że dotyczy to __na instancję__, a nie łącznych zasobów w klastrze**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Powyższe tabele to ostrożne szacunki. Rzeczywiste wymagania mogą się różnić w zależności od Twojej konfiguracji (rozmiary stron, wolumen komentarzy itp.).