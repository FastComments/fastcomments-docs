Wszystkie zmiany obiektu Comment w systemie wywołują zdarzenie, które trafia do kolejki.

Początkowe zdarzenie webhook jest zwykle wysyłane w ciągu sześciu sekund od wystąpienia źródła zdarzenia.

Możesz monitorować tę kolejkę w panelu administracyjnym Webhooks na wypadek, gdyby Twoje API przestało działać.

Jeśli żądanie do Twojego API się nie powiedzie, ponownie umieścimy je w kolejce zgodnie z harmonogramem.

Ten harmonogram to `1 Minute * the retry count`. Jeśli wywołanie zakończy się niepowodzeniem raz, spróbuje ponownie za
minutę. Jeśli nie powiedzie się dwa razy, poczeka wtedy dwie minuty, i tak dalej. Dzięki temu
nie przeciążymy Twojego API, jeśli przestaje działać z powodów związanych z obciążeniem.

Webhooks można anulować z [strony logów](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).