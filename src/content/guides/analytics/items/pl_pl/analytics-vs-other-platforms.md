Możesz zauważyć, że nasze metryki Analytics pokazują nieco inne liczby niż np. Google Ads © lub podobne produkty.

Dla witryn z jednym widgetem komentarzy na stronę, liczby podawane przez FastComments Analytics są bardzo dokładne, a jeśli są nieprawidłowe, będą **niższe** niż rzeczywista wartość, ale nie więcej.

Jeśli masz SPA, możesz zauważyć, że liczby FastComments Analytics są wyższe niż te raportowane przez Twoje produkty marketingowe. Dzieje się tak, ponieważ produkt marketingowy może śledzić tylko wtedy, gdy strona nie jest załadowana, ale nie za każdym razem, gdy użytkownik robi coś na stronie, co może wywołać wyświetlenie nowego wątku komentarzy, co liczyłoby się jako ładowanie strony dla FastComments Analytics.

#### Informacje techniczne

FastComments Analytics śledzi każde ładowanie strony i nie polega na losowości jako optymalizacji. Każde ładowanie strony powoduje aktualizację licznika w pamięci w każdym wątku na każdym serwerze, który jest następnie okresowo zapisywany w bazie danych za pomocą operacji atomowej.
