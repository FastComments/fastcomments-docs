---
Domyślnie FastComments nie ogranicza języków używanych w komentarzach. 

Może być pożądane ograniczenie języków, które społeczność używa.

Można to skonfigurować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Selektor dozwolonych języków na stronie dostosowywania widgetu, służący do ograniczenia, jakich języków mogą używać komentarze'; title='Dozwolone języki' app-screenshot-end]

System przetworzy ich komentarz, określi jego język, a następnie dopasuje go do listy dozwolonych.

Jeśli komentarz zostanie napisany w języku, który nie jest dozwolony, wyświetlony zostanie zlokalizowany komunikat o błędzie. 

---