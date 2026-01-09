Som standard synkroniserer FastComments tilbage til dit WordPress-websted dagligt. Dette er udelukkende til backupformål, så du fortsat ejer en kopi af dataene, og for plugins
der måtte være afhængige af det.

Dette sker ikke øjeblikkeligt ved hver gemt kommentar på grund af, at nogle websteder kan håndtere stor læsetrafik, men deres databaseimplementeringer ikke altid kan håndtere den store skrivetrafik (derfor uddelegeres dette arbejde til FastComments).

Synkroniseringsplanen tilbage til WordPress kan tilpasses ved at installere et plugin. Vi anbefaler [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Installer WP Crontrol
2. Gå til `Settings -> Cron Schedules`.
3. Gå til fanen `Cron Events`.
4. Søg efter `fastcomments_cron_hook`.
5. Rediger begivenheden. Du kan konfigurere hooken til at køre timevis, to gange om dagen, dagligt (standard), eller en gang om ugen.

Synkroniseringen tilbage til WordPress kan også udføres manuelt når som helst ved at gå til FastComments-pluginets dashboard og vælge `Manually Sync`. Du får
muligheden for at synkronisere tilbage til din WP-installation, eller at gen-uploade dine WP-kommentarer til FastComments-serverne.