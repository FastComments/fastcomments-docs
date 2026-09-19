[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments nie śledzi, kto obejrzał każdy komentarz ani nie dostarcza żadnych statystyk na ten temat.

Możemy jednak włączyć tę funkcję, a system zacznie śledzić, gdy każdy użytkownik przewija do komentarza.

Gdy to nastąpi, licznik obok ikony oka wyświetlanej przy każdym komentarzu zostanie zwiększony. Licznik jest aktualizowany na żywo i skracany zgodnie z lokalizacją użytkownika.

Możemy to włączyć, ustawiając flagę **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Włączanie liczby wyświetleń komentarzy'; code-example-end]

Można to dostosować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Strona dostosowywania widgetu z zaznaczonym polem wyboru liczby wyświetleń, tak aby każdy komentarz wyświetlał ikonę oka i liczbę'; title='Włączanie liczby wyświetleń komentarzy' app-screenshot-end]

Śledzimy identyfikator użytkownika* który obejrzał komentarz przez tydzień, tak aby przy ponownym obejrzeniu komentarza w tym tygodniu licznik się nie zwiększał. Jeśli obejrzysz komentarz ponownie po upływie tygodnia, licznik zwiększy się ponownie.

- *Uwaga: lub anonimowy identyfikator sesji, albo IP użytkownika jako wartość haszowaną.