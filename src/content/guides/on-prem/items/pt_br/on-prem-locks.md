---
Como qualquer sistema distribuído, o FastComments precisa de alguma forma de bloquear recursos e procedimentos. Esses bloqueios podem ser monitorados via o endpoint `/locks-in-progress`.

[Por exemplo, aqui está o endpoint do nosso shard dos EUA](https://fastcomments.com/locks-in-progress).

Isto pode ser útil para ver por que o sistema está travado ou sob carga. Se, por exemplo, um SRE quiser saber por que o sistema está apresentando alta carga de CPU, ele poderia
verificar este endpoint para obter o nome do cron que está se comportando mal.

---