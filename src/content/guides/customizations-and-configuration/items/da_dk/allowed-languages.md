Som standard begrænser FastComments ikke de sprog, der bruges til at kommentere.

Det kan være ønskeligt at begrænse de sprog, et community bruger.

Dette kan konfigureres uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Vælgeren for tilladte sprog på widget-tilpasningssiden til at begrænse, hvilke sprog kommentarer må bruge'; title='Tilladte sprog' app-screenshot-end]

Systemet vil analysere kommentaren, bestemme dens sprog og derefter matche den med listen over tilladte sprog.

Hvis kommentaren er skrevet på et sprog, der ikke er tilladt, vises en lokaliseret fejlmeddelelse.