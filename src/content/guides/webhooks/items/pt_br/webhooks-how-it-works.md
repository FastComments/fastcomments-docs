Todas as alterações no objeto Comment no sistema disparam um evento que acaba em uma fila.

O evento inicial de webhook geralmente é enviado dentro de seis segundos após a ocorrência da fonte do evento.

Você pode monitorar essa fila no painel de Webhooks no caso de sua API ficar fora do ar.

Se uma solicitação para sua API falhar, nós a colocaremos novamente na fila conforme uma programação.

Essa programação é `1 Minute * the retry count`. Se a chamada falhar uma vez, ela tentará novamente em
um minuto. Se falhar duas vezes, então aguardará dois minutos, e assim por diante. Isso é para que não
sobrecarreguemos sua API caso ela esteja ficando indisponível por motivos relacionados à carga.

Os Webhooks podem ser cancelados a partir da [página de logs](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).