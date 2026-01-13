---
Todos los cambios en el objeto Comment en el sistema disparan un evento que termina en una cola.

El evento webhook inicial suele enviarse dentro de seis segundos desde que ocurre la fuente del evento.

Puedes supervisar esta cola en el panel de administración de Webhooks en caso de que tu API se caiga.

Si una solicitud a tu API falla, la volveremos a encolar según un programa.

Ese programa es `1 Minute * the retry count`. Si la llamada falla una vez, intentará de nuevo en
un minuto. Si falla dos veces, esperará entonces dos minutos, y así sucesivamente. Esto es para que no
sobrecarguemos tu API si estáis cayendo por razones relacionadas con la carga.

Los webhooks pueden cancelarse desde la [página de registros](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).

---