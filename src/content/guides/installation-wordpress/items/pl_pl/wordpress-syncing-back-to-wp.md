---
Domyślnie FastComments synchronizuje dane z powrotem z Twoją stroną WordPress codziennie. Służy to wyłącznie celom tworzenia kopii zapasowej, abyś nadal miał kopię danych, oraz dla wtyczek, które mogą od tego zależeć.

To nie dzieje się natychmiast przy każdym zapisanym komentarzu ze względu na to, że niektóre strony potrafią obsługiwać duży ruch odczytu, a ich wdrożenia baz danych nie zawsze radzą sobie z dużym ruchem zapisu (stąd oddelegowanie tej pracy do FastComments).

Harmonogram synchronizacji z powrotem do WordPress można dostosować, instalując wtyczkę. Zalecamy [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

Synchronizację z powrotem do WordPress można również wykonać ręcznie w dowolnym momencie, przechodząc do panelu wtyczki FastComments i wybierając `Manually Sync`. Będziesz miał opcję zsynchronizowania danych z powrotem do swojej instalacji WP lub ponownego przesłania komentarzy WP na serwery FastComments.
---