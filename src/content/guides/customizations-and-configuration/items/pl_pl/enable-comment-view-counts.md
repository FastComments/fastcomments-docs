[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie śledzi, kto obejrzał dany komentarz ani nie dostarcza żadnych statystyk w tym zakresie.

Możemy jednak włączyć tę funkcję — wówczas system zacznie śledzić, gdy użytkownik przewinie do komentarza.

Kiedy to nastąpi, licznik obok ikony oka przy każdym komentarzu zostanie zwiększony. Licznik jest aktualizowany na żywo i skracany zgodnie z ustawieniami regionalnymi użytkownika.

Możemy to włączyć, ustawiając flagę **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Włączanie liczników wyświetleń komentarzy'; code-example-end]

Można to dostosować bez kodu, na stronie personalizacji widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Włączanie liczników wyświetleń komentarzy' app-screenshot-end]

Śledzimy user id* osoby, która obejrzała komentarz, dzięki czemu ponowne obejrzenie komentarza przez tę samą osobę nie zwiększa licznika. Jeśli komentarz zostanie obejrzany ponownie po upływie dwóch lat, licznik zostanie ponownie zwiększony.

- *Uwaga: albo anon session id, albo user's IP jako wartość haszowana.