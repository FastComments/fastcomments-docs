---
Som standard begrænser FastComments ikke de sprog, der bruges til at kommentere. 

Det kan være ønskeligt at begrænse de sprog, et fællesskab bruger.

Dette kan konfigureres uden kode på siden til tilpasning af widgeten:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

Systemet vil analysere kommentaren og bestemme sproget, og derefter sammenligne det med den tilladte liste.

Hvis kommentaren er skrevet på et sprog, der ikke er tilladt, vises en lokaliseret fejlmeddelelse. 

---