FastComments suporta webhooks apenas para o recurso Comentário.

Nós suportamos webhooks para criação, remoção e atualização de comentários.

Cada um desses é considerado um evento separado em nosso sistema e, como tal, possui semânticas e estruturas diferentes para os eventos de webhook.

Qualquer número de endpoints pode se inscrever no mesmo evento, a partir do painel ou através da API (veja Gerenciando Webhooks via API). Cada webhook é entregue de forma independente.