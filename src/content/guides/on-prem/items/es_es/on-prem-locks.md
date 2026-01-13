Como cualquier sistema distribuido, FastComments necesita alguna forma de bloquear recursos y procedimientos. Estos bloqueos pueden supervisarse mediante el endpoint `/locks-in-progress`.

[Por ejemplo, aquí está el endpoint en nuestro shard de EE. UU.](https://fastcomments.com/locks-in-progress).

Esto puede ser útil para ver por qué el sistema está atascado o bajo carga. Si, por ejemplo, un SRE quiere saber por qué el sistema está experimentando una alta carga de CPU, podría
consultar este endpoint para obtener el nombre del cron que se está comportando mal.