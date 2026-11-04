Domyślnie FastComments synchronizuje dane z powrotem do Twojej witryny WordPress codziennie. Służy to wyłącznie celom tworzenia kopii zapasowych, dzięki czemu nadal posiadasz kopię danych, a także dla wtyczek, które mogą od tego zależeć.

Nie dzieje się to natychmiast przy każdym zapisanym komentarzu, ponieważ ze względu na naturę niektórych witryn, które są w stanie obsłużyć duży ruch odczytu, ich wdrożenia baz danych nie zawsze są w stanie obsłużyć duży ruch zapisu (stąd przeniesienie tej pracy do FastComments).

Harmonogram synchronizacji z powrotem do WordPressa można dostosować, instalując wtyczkę. Polecamy [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Kroki:

1. Zainstaluj WP Crontrol
2. Przejdź do `Settings -> Cron Schedules`.
3. Przejdź do zakładki `Cron Events`.
4. Wyszukaj `fastcomments_cron_hook`.
5. Edytuj zdarzenie. Możesz skonfigurować hook tak, aby uruchamiał się co godzinę, dwa razy dziennie, codziennie (domyślnie) lub raz w tygodniu.

Synchronizację z powrotem do WordPressa można również wykonać w dowolnym momencie ręcznie, przechodząc do panelu wtyczki FastComments i wybierając `Manually Sync`. Będziesz mieć opcję synchronizacji z powrotem do swojej instalacji WP lub ponownego przesłania komentarzy WP na serwery FastComments.