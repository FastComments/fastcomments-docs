Domyślnie FastComments automatycznie wykryje, czy Twoja strona ma ciemne tło na podstawie "odległości od czerni" w kole kolorów.

Nasze produkty robią, co w ich mocy, jednak jest prawie nieskończona liczba kolorów w kole kolorów i mogą istnieć scenariusze, w których aplikacja wybiera tryb ciemny, gdy nie jest to odpowiednie, i odwrotnie. Ta dokumentacja opisuje, jak uzyskać bardziej szczegółową kontrolę nad tym.

#### Szczegóły techniczne

Wykrywamy tryb ciemny, przechodząc przez elementy na stronie w górę od widgetu komentarzy, szukając ciemnego tła podczas początkowego ładowania widgetu.

Aby przełączyć tryb ciemny po tym kroku, musisz wywołać widget, aby zaktualizować jego konfigurację. Jest to opisane w sekcji `Konfiguracja ręczna`.
