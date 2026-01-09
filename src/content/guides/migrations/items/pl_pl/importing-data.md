Chociaż pomoc techniczna FastComments może pomóc przy migracjach, większość z nich można przeprowadzić i monitorować łatwo bez udziału personelu wsparcia.

Natywnie obsługujemy import eksportów od następujących dostawców:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

Przechodząc [tutaj](https://fastcomments.com/auth/my-account/manage-data/import) możemy przesłać plik zawierający dane do migracji.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitorowanie importów

FastComments wykorzystuje system przetwarzania zadań do obsługi importów i eksportów. Gdy system rozpocznie przetwarzanie Twojego zadania, będzie okresowo raportować jego status w interfejsie importu lub eksportu.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Zauważ, że statusy importów i eksportów są widoczne dla wszystkich administratorów na koncie.

Jeśli Twoje zadanie nie powiedzie się, nie zostanie automatycznie ponownie uruchomione. Import będzie trzeba spróbować ponownie. Jeśli jakikolwiek import lub eksport zakończy się niepowodzeniem,
nasi administratorzy systemu zostaną automatycznie powiadomieni. Jeśli zidentyfikujemy problem, skontaktujemy się z Tobą, aby sprawdzić, czy możemy pomóc.

### Ponowne uruchamianie importu

Podczas niektórych migracji konieczne jest uruchomienie importu wielokrotnie. Na przykład często wykonuje się pierwsze przejście migracji w celach testowych, a następnie ponownie uruchamia import z najnowszymi danymi przed dokonaniem przełączenia.

Ponowny import tych samych treści **nie spowoduje powstania duplikatów**.

### Bezpieczeństwo danych i wygaśnięcie

Pliki importu nie są w żaden sposób dostępne za pośrednictwem zewnętrznych żądań, a pliki importu są usuwane z naszego systemu, jak tylko
import zostanie zakończony.