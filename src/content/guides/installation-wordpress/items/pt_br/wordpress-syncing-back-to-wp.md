Por padrão, o FastComments sincroniza de volta com seu site WordPress diariamente. Isso é puramente para fins de backup, para que você continue a possuir uma cópia dos dados, e para plugins que possam depender deles.

Isso não acontece imediatamente com cada comentário salvo devido à natureza de alguns sites: embora possam suportar grande tráfego de leitura, suas implantações de banco de dados nem sempre conseguem lidar com alto volume de gravações (por isso esse trabalho é delegado ao FastComments).

O agendamento da sincronização de volta ao WordPress pode ser personalizado instalando um plugin. Recomendamos [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Passos:

1. Instale o WP Crontrol
2. Vá para `Settings -> Cron Schedules`.
3. Vá para a guia `Cron Events`.
4. Procure por `fastcomments_cron_hook`.
5. Edite o evento. Você pode configurar o hook para ser executado a cada hora, duas vezes ao dia, diariamente (padrão), ou uma vez por semana.

A sincronização de volta ao WordPress também pode ser realizada manualmente a qualquer momento indo ao painel do plugin FastComments e selecionando `Manually Sync`. Você terá a opção de sincronizar de volta para sua instalação do WP, ou de reenviar seus comentários do WP para os servidores do FastComments.